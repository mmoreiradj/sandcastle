use crate::{
    crd::SandcastleProject,
    error::SandcastleError,
    operator::{
        helm::{Helm, HelmCli},
        metrics::Metrics,
    },
};
use chrono::{DateTime, Utc};
use futures::StreamExt;
use kube::{
    Api, Client, ResourceExt,
    api::ListParams,
    runtime::{
        Controller,
        controller::Action,
        events::{Recorder, Reporter},
        finalizer,
    },
};
use serde::Serialize;
use std::{sync::Arc, time::Duration};
use tokio::sync::RwLock;
use tracing::{Span, instrument, warn};
mod helm;
mod metrics;
mod reconcile;

#[derive(Debug, Serialize, Clone)]
/// The diagnostics for the operator.
///
/// This is used to track the last event and the reporter.
pub struct Diagnostics {
    #[serde(deserialize_with = "from_ts")]
    pub last_event: DateTime<Utc>,
    #[serde(skip)]
    pub reporter: Reporter,
}

impl Diagnostics {
    fn recorder(&self, client: Client) -> Recorder {
        Recorder::new(client, self.reporter.clone())
    }
}

/// State shared between the controller and the web server.
#[derive(Clone)]
pub struct AppState {
    diagnostics: Arc<RwLock<Diagnostics>>,
    metrics: Arc<Metrics>,
}

impl AppState {
    /// Get the metrics as a string.
    pub fn metrics(&self) -> String {
        let mut buffer = String::new();
        let registry = &*self.metrics.registry;
        prometheus_client::encoding::text::encode(&mut buffer, registry).unwrap();
        buffer
    }

    /// Get the diagnostics.
    pub async fn diagnostics(&self) -> Diagnostics {
        self.diagnostics.read().await.clone()
    }

    /// Create a controller context that can update the state.
    pub async fn controller_context(&self, client: Client) -> Arc<Context<HelmCli>> {
        Arc::new(Context {
            client: client.clone(),
            recorder: self.diagnostics.read().await.recorder(client),
            metrics: self.metrics.clone(),
            diagnostics: self.diagnostics.clone(),
            helm: HelmCli::default(),
        })
    }
}

/// The context for the operator.
///
/// This is passed to all the components of the operator.
#[derive(Clone)]
pub struct Context<HELM: Helm> {
    pub client: Client,
    pub recorder: Recorder,
    pub diagnostics: Arc<RwLock<Diagnostics>>,
    pub metrics: Arc<Metrics>,
    pub helm: HELM,
}

const SANDCASTLE_ENVIRONMENT_FINALIZER: &str = "environment.sandcastle.dev";

#[instrument(skip(environment, context), fields(trace_id))]
pub async fn reconcile<HELM: Helm>(
    environment: Arc<SandcastleProject>,
    context: Arc<Context<HELM>>,
) -> Result<Action, SandcastleError> {
    let trace_id = sandcastle_telemetry::get_trace_id();
    if trace_id != opentelemetry::trace::TraceId::INVALID {
        Span::current().record("trace_id", tracing::field::display(trace_id));
    }
    let _timer = context.metrics.reconcile.count_and_measure(&trace_id);
    context.diagnostics.write().await.last_event = Utc::now();
    // we can unwrap because the worker_group is namespace scoped
    let ns = environment.namespace().unwrap();
    let environments = Api::<SandcastleProject>::namespaced(context.client.clone(), &ns);

    tracing::info!(
        "reconciling environment \"{}\" in ns \"{}\"",
        environment.name_any(),
        ns
    );
    Ok(finalizer(
        &environments,
        SANDCASTLE_ENVIRONMENT_FINALIZER,
        environment,
        |event| async {
            match event {
                finalizer::Event::Apply(e) => e.reconcile(context.clone()).await,
                finalizer::Event::Cleanup(e) => e.cleanup(context.clone()).await,
            }
        },
    )
    .await
    .unwrap())
}

fn error_policy<HELM: Helm>(
    environment: Arc<SandcastleProject>,
    error: &SandcastleError,
    context: Arc<Context<HELM>>,
) -> Action {
    let name = environment.name_any();
    let namespace = environment.namespace().unwrap();
    warn!("reconcile failed for environment \"{namespace}/{name}\": {error:?}");
    context.metrics.reconcile.set_failure(&*environment, error);
    Action::requeue(Duration::from_secs(5 * 60))
}

pub async fn start_operator(
    client: Client,
    watcher_config: kube::runtime::watcher::Config,
    state: AppState,
) {
    let environments = Api::<SandcastleProject>::all(client.clone());

    if let Err(e) = environments.list(&ListParams::default().limit(1)).await {
        tracing::error!(
            "failed to list environments: {e:?}, check if the crd is properly installed and roles are properly configured"
        );
        return;
    }
    Controller::new(environments, watcher_config.clone())
        .shutdown_on_signal()
        .run(
            reconcile::<HelmCli>,
            error_policy,
            state.controller_context(client).await,
        )
        .filter_map(|x| async move { std::result::Result::ok(x) })
        .for_each(|_| async {})
        .await;
}

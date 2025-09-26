use std::sync::Arc;

use crate::{crd::SandcastleProject, error::SandcastleError, operator::metrics::Metrics};
use chrono::{DateTime, Utc};
use kube::{
    Api, Client, ResourceExt,
    runtime::{
        controller::Action,
        events::{Recorder, Reporter},
        finalizer,
    },
};
use serde::Serialize;
use tokio::sync::RwLock;
use tracing::{Span, instrument};
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
    pub async fn controller_context(&self, client: Client) -> Arc<Context> {
        Arc::new(Context {
            client: client.clone(),
            recorder: self.diagnostics.read().await.recorder(client),
            metrics: self.metrics.clone(),
            diagnostics: self.diagnostics.clone(),
        })
    }
}

/// The context for the operator.
///
/// This is passed to all the components of the operator.
#[derive(Clone)]
pub struct Context {
    pub client: Client,
    pub recorder: Recorder,
    pub diagnostics: Arc<RwLock<Diagnostics>>,
    pub metrics: Arc<Metrics>,
}

const SANDCASTLE_ENVIRONMENT_FINALIZER: &str = "environment.sandcastle.dev";

#[instrument(skip(environment, context), fields(trace_id))]
pub async fn reconcile(
    environment: Arc<SandcastleProject>,
    context: Arc<Context>,
) -> Result<Action, kube::runtime::finalizer::Error<SandcastleError>> {
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
    finalizer(
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
}

pub fn start_operator() {}

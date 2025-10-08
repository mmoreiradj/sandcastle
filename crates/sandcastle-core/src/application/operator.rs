use std::{sync::Arc, time::Duration};

use futures::StreamExt;
use k8s_openapi::api::core::v1::Secret;
use kube::{
    Api, Client, ResourceExt,
    api::ListParams,
    runtime::{Controller, controller::Action, finalizer},
};
use snafu::ResultExt;
use tracing::instrument;

use crate::{
    Result,
    domain::repositories::{
        models::RepositoryConfiguration, ports::RepositoryConfigurationService,
        services::RepositoryConfigurations,
    },
    error::{FinalizerSnafu, SandcastleError},
    infrastructure::repo_config_service::GithubAppSecretData,
};

#[derive(Clone)]
pub struct OperatorContext {
    pub client: Client,
    pub repository_configuration_service: RepositoryConfigurations,
    pub namespace: String,
}

const SANDCASTLE_FINALIZER: &str = "sandcastle.dev/finalizer";

async fn apply(secret: Arc<Secret>, context: Arc<OperatorContext>) -> Result<Action> {
    let github_app_secret_data = GithubAppSecretData::from_secret(secret)?;
    let repository_config = RepositoryConfiguration::from(github_app_secret_data);
    context
        .repository_configuration_service
        .upsert_repository_configuration(repository_config)
        .await?;
    Ok(Action::requeue(Duration::from_secs(5 * 60)))
}

async fn cleanup(secret: Arc<Secret>, context: Arc<OperatorContext>) -> Result<Action> {
    context
        .repository_configuration_service
        .delete_repository_configuration(&secret.name_any())
        .await?;
    Ok(Action::requeue(Duration::from_secs(5 * 60)))
}

#[instrument(skip(secret, context), fields(_secret_name))]
async fn reconcile(secret: Arc<Secret>, context: Arc<OperatorContext>) -> Result<Action> {
    let _secret_name = &secret.name_any();
    tracing::info!("Reconciling secret");
    let secrets = Api::<Secret>::namespaced(context.client.clone(), &context.namespace);

    finalizer(&secrets, SANDCASTLE_FINALIZER, secret, |event| async move {
        match event {
            finalizer::Event::Apply(secret) => apply(secret, context.clone()).await,
            finalizer::Event::Cleanup(secret) => cleanup(secret, context.clone()).await,
        }
    })
    .await
    .context(FinalizerSnafu)
}

#[instrument(skip(secret, error, _context), fields(_secret_name))]
fn error_policy(
    secret: Arc<Secret>,
    error: &SandcastleError,
    _context: Arc<OperatorContext>,
) -> Action {
    let _secret_name = secret.name_any();
    tracing::warn!("Failed reconciling secret {:?}", error);
    Action::requeue(Duration::from_secs(5 * 60))
}

pub async fn run(client: Client, context: OperatorContext) {
    let secrets = Api::<Secret>::namespaced(client.clone(), &context.namespace);

    if let Err(e) = secrets.list(&ListParams::default().limit(1)).await {
        tracing::error!("CRD is not queryable; {e:?}. Is the CRD installed?");
        std::process::exit(1);
    }
    let watcher_config =
        kube::runtime::watcher::Config::default().labels("sandcastle.dev/secret-type=repository");
    Controller::new(secrets, watcher_config.clone())
        .shutdown_on_signal()
        .run(reconcile, error_policy, Arc::new(context.clone()))
        .filter_map(|x| async move { std::result::Result::ok(x) })
        .for_each(|_| futures::future::ready(()))
        .await;
}

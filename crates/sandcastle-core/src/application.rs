use kube::{Client, Config};

use crate::domain::repositories::services::{
    DefaultRepositoryConfigurationService, RepositoryConfigurations,
};

mod http;
mod operator;

/// State shared beetween the HTTP and Operator
pub(crate) struct ApplicationState {
    pub(crate) kube_client: Client,
    pub(crate) namespace: String,
    pub(crate) repository_configuration_service: RepositoryConfigurations,
}

pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let kube_client = Client::try_default().await?;
    let config = Config::infer().await?;
    let context = operator::OperatorContext {
        client: kube_client.clone(),
        repository_configuration_service: DefaultRepositoryConfigurationService::default().into(),
        namespace: config.default_namespace,
    };
    tokio::select! {
      _ = http::start() => {
        tracing::info!("HTTP server started");
        Ok(())
      }
      _ = operator::run(kube_client, context) => {
        tracing::info!("Operator started");
        Ok(())
      }
    }
}

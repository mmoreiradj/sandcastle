use kube::{Client, Config};

use crate::domain::repositories::services::{
    DefaultRepositoryConfigurationService, RepositoryConfigurations,
};

mod http;
mod operator;
pub(crate) mod reconciliation;

/// State shared beetween the HTTP and Operator
#[derive(Clone)]
pub(crate) struct ApplicationState {
    pub(crate) kube_client: Client,
    pub(crate) namespace: String,
    pub(crate) repository_configuration_service: RepositoryConfigurations,
}

impl ApplicationState {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let kube_client = Client::try_default().await?;
        let config = Config::infer().await?;
        Ok(Self {
            kube_client,
            namespace: config.default_namespace,
            repository_configuration_service: DefaultRepositoryConfigurationService::default().into(),
        })
    }

    pub fn operator_context(&self) -> operator::OperatorContext {
        operator::OperatorContext {
            client: self.kube_client.clone(),
            repository_configuration_service: self.repository_configuration_service.clone(),
            namespace: self.namespace.clone(),
        }
    }
}

pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let state = ApplicationState::new().await?;
    let context = state.operator_context();
    tokio::select! {
      _ = http::start(state.clone()) => {
        tracing::info!("HTTP server started");
        Ok(())
      }
      _ = operator::run(context) => {
        tracing::info!("Operator started");
        Ok(())
      }
    }
}

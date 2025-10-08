use kube::{Client, Config};

use crate::domain::repositories::services::DefaultRepositoryConfigurationService;

mod http;
mod operator;

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

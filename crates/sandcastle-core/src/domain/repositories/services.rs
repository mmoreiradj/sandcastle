use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use tokio::sync::RwLock;
use tracing::instrument;

use crate::Result;
use crate::domain::repositories::{
    models::RepositoryConfiguration, ports::RepositoryConfigurationService,
};

#[cfg(test)]
use crate::domain::repositories::ports::MockRepositoryConfigurationService;

#[enum_dispatch(RepositoryConfigurationService)]
#[derive(Clone)]
pub enum RepositoryConfigurations {
    DefaultRepositoryConfigurationService,
    #[cfg(test)]
    MockRepositoryConfigurationService,
}

#[derive(Clone)]
pub struct DefaultRepositoryConfigurationService {
    pub repository_configurations: Arc<RwLock<HashMap<String, RepositoryConfiguration>>>,
}

impl Default for DefaultRepositoryConfigurationService {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultRepositoryConfigurationService {
    pub fn new() -> Self {
        Self {
            repository_configurations: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl RepositoryConfigurationService for DefaultRepositoryConfigurationService {
    #[instrument(skip(self))]
    async fn get_repository_configuration(
        &self,
        repository_url: &str,
    ) -> Result<Option<RepositoryConfiguration>> {
        tracing::debug!("Getting repository configuration for {}", repository_url);
        Ok(self
            .repository_configurations
            .read()
            .await
            .get(repository_url)
            .cloned())
    }

    #[instrument(skip(self, repository_configuration), fields(repository_url = %repository_configuration.repository_url))]
    async fn upsert_repository_configuration(
        &self,
        repository_configuration: RepositoryConfiguration,
    ) -> Result<()> {
        tracing::debug!("Upserting repository configuration for {}", repository_configuration.repository_url);
        self.repository_configurations.write().await.insert(
            repository_configuration.repository_url.clone(),
            repository_configuration,
        );
        Ok(())
    }

    #[instrument(skip(self), fields(repository_url))]
    async fn delete_repository_configuration(&self, repository_url: &str) -> Result<()> {
        tracing::debug!("Deleting repository configuration for {}", repository_url);
        self.repository_configurations
            .write()
            .await
            .remove(repository_url);
        Ok(())
    }
}

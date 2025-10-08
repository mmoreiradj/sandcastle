use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use tokio::sync::RwLock;

use crate::Result;
use crate::domain::repositories::{
    models::RepositoryConfiguration, ports::RepositoryConfigurationService,
};

#[enum_dispatch(RepositoryConfigurationService)]
#[derive(Clone)]
pub enum RepositoryConfigurations {
    DefaultRepositoryConfigurationService,
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
    async fn get_repository_configuration(
        &self,
        repository_url: &str,
    ) -> Result<Option<RepositoryConfiguration>> {
        Ok(self
            .repository_configurations
            .read()
            .await
            .get(repository_url)
            .cloned())
    }

    async fn upsert_repository_configuration(
        &self,
        repository_configuration: RepositoryConfiguration,
    ) -> Result<()> {
        self.repository_configurations.write().await.insert(
            repository_configuration.repository_url.clone(),
            repository_configuration,
        );
        Ok(())
    }

    async fn delete_repository_configuration(&self, repository_url: &str) -> Result<()> {
        self.repository_configurations
            .write()
            .await
            .remove(repository_url);
        Ok(())
    }
}

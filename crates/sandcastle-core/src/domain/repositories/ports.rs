use async_trait::async_trait;
use enum_dispatch::enum_dispatch;

use crate::{Result, domain::repositories::models::RepositoryConfiguration};

/// A trait to fetch a repository's configuration
#[async_trait]
#[enum_dispatch]
pub trait RepositoryConfigurationService: Clone + Send + Sync {
    async fn get_repository_configuration(
        &self,
        repository_url: &str,
    ) -> Result<Option<RepositoryConfiguration>>;
    async fn upsert_repository_configuration(
        &self,
        repository_configuration: RepositoryConfiguration,
    ) -> Result<()>;
    async fn delete_repository_configuration(&self, repository_url: &str) -> Result<()>;
}

use async_trait::async_trait;

use crate::{domain::environment::ports::GitOpsPlatformService, error::SandcastleError};

#[derive(Clone)]
pub struct ArgoCD;

#[async_trait]
impl GitOpsPlatformService for ArgoCD {
    async fn create_or_update_application(&self, application: &str) -> Result<(), SandcastleError> {
        Ok(())
    }

    async fn delete_application(&self, application: &str) -> Result<(), SandcastleError> {
        Ok(())
    }
}

use async_trait::async_trait;
use enum_dispatch::enum_dispatch;

use crate::{
    domain::environment::models::{DownloadFileRequest, ReconcileContext},
    error::SandcastleError,
};

/// Reconcile the environment
#[async_trait]
pub trait Reconcile: Clone + Send + Sync {
    async fn reconcile(&self, context: ReconcileContext) -> Result<(), SandcastleError>;
}

/// A trait for a GitOps platform services
/// This is supposed to be argocd, flux, etc.
#[async_trait]
#[enum_dispatch]
pub trait GitOpsPlatformService: Clone + Send + Sync {
    async fn create_or_update_application(&self, application: &str) -> Result<(), SandcastleError>;
    async fn delete_application(&self, application: &str) -> Result<(), SandcastleError>;
}

/// A trait for a VCS service
/// This is supposed to be github, gitlab, etc.
#[async_trait]
#[enum_dispatch]
pub trait VCSService: Clone + Send + Sync {
    async fn download_file(&self, request: DownloadFileRequest)
    -> Result<Vec<u8>, SandcastleError>;
}

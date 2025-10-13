use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use mockall::mock;

use crate::{
    domain::environment::models::{
        DownloadFileRequest, FetchPRLastCommitSHARequest, ReconcileContext,
    },
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
    async fn download_file(&self, request: DownloadFileRequest) -> Result<String, SandcastleError>;
    async fn fetch_pr_last_commit_sha(
        &self,
        request: FetchPRLastCommitSHARequest,
    ) -> Result<String, SandcastleError>;
}

mock! {
    pub VCSService {}

    #[async_trait]
    impl VCSService for VCSService {
        async fn download_file(&self, request: DownloadFileRequest) -> Result<String, SandcastleError>;
        async fn fetch_pr_last_commit_sha(&self, request: FetchPRLastCommitSHARequest) -> Result<String, SandcastleError>;
    }

    impl Clone for VCSService {
        fn clone(&self) -> Self {
            self.clone()
        }
    }
}

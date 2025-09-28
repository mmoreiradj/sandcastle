
use crate::{domain::environment::{ports::Reconcile, services::{GitOpsPlatform, VCS}}, error::SandcastleError};

#[derive(Clone)]
pub struct ReconcileContext {
    /// The ID of the reconcile context
    pub id: String,
    /// The VCS context
    pub vcs: VcsContext,
    /// The VCS service
    pub vcs_service: VCS,
    /// The GitOps platform service
    pub gitops_platform_service: GitOpsPlatform,
}


#[derive(Debug, Clone)]
pub struct VcsContext {
    pub repository: RepositoryContext,
    pub pull_request: PullRequestContext,
    pub comment: CommentContext,
}

#[derive(Debug, Clone)]
pub struct RepositoryContext {
    /// The name of the repository
    pub name: String,
    /// The full name of the repository (owner/name)
    pub full_name: String,
    /// Whether the repository is private
    pub private: bool,
    /// The base URI of the repository
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct PullRequestContext {
    pub number: u64,
    pub title: String,
}

#[derive(Debug, Clone)]
pub struct CommentContext {
    pub body: String,
}

/// Action to create or update a GitOps Application
#[derive(Debug, Clone)]
pub struct CreateOrUpdateArgocdApplicationAction {
    pub application: String,
}

/// Action to delete an Argocd Application
#[derive(Debug, Clone)]
pub struct DeleteArgocdApplicationAction {
    /// The GitOps Application to delete
    pub application: String,
}

#[derive(Debug, Clone)]
pub enum ReconcileActions {
    CreateOrUpdateArgocdApplication(CreateOrUpdateArgocdApplicationAction),
    DeleteArgocdApplication(DeleteArgocdApplicationAction),
}

impl ReconcileActions {
    pub async fn reconcile(&self, context: ReconcileContext) -> Result<(), SandcastleError> {
        match self {
            ReconcileActions::CreateOrUpdateArgocdApplication(action) => action.reconcile(context).await,
            ReconcileActions::DeleteArgocdApplication(action) => action.reconcile(context).await,
        }
    }
}

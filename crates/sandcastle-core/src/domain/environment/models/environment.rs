use crate::{
    domain::environment::{
        models::config::{BuiltinConfigKey, ConfigPath, SandcastleConfiguration},
        ports::Reconcile,
        services::{GitOpsPlatform, VCS},
    },
    error::SandcastleError,
};

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
    /// Sandcastle configuration
    pub config: SandcastleConfiguration,
}

impl ReconcileContext {
    pub fn get_config_value(&self, path: &str) -> Option<String> {
        let config_path = ConfigPath::parse(path)?;

        match config_path {
            ConfigPath::Builtin(key) => self.get_builtin_config_value(&key),
            ConfigPath::Custom(parts) => self.config.get_custom_value(&parts),
        }
    }

    fn get_builtin_config_value(&self, key: &BuiltinConfigKey) -> Option<String> {
        match key {
            BuiltinConfigKey::EnvironmentName => Some(self.vcs.repository.name.clone()),
            BuiltinConfigKey::RepoURL => Some(self.vcs.repository.url.clone()),
            BuiltinConfigKey::TargetRevision => Some(self.vcs.pull_request.last_commit_sha.clone()),
            BuiltinConfigKey::LastCommitSHA => Some(self.vcs.pull_request.last_commit_sha.clone()),
        }
    }
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
    pub last_commit_sha: String,
}

#[derive(Debug, Clone)]
pub struct CommentContext {
    pub body: String,
}

/// Action to create or update a GitOps Application
#[derive(Debug, Clone)]
pub struct CreateOrUpdateArgocdApplicationAction {
    /// The GitOps File
    pub application: String,
}

/// Action to delete an Argocd Application
#[derive(Debug, Clone)]
pub struct DeleteArgocdApplicationAction {
    /// The GitOps File
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
            ReconcileActions::CreateOrUpdateArgocdApplication(action) => {
                action.reconcile(context).await
            }
            ReconcileActions::DeleteArgocdApplication(action) => action.reconcile(context).await,
        }
    }
}

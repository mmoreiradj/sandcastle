use octocrab::{models::{webhook_events::{WebhookEvent, WebhookEventPayload}, Repository}, Octocrab};
use serde_yaml::Value;
use crate::{application::ApplicationState, domain::environment::{models::{DownloadFileRequest, FetchPRLastCommitSHARequest}, services::GitHub}, Result};
use crate::{
    domain::environment::{
        models::config::{BuiltinConfigKey, ConfigPath, SandcastleConfiguration},
        ports::{Reconcile, VCSService},
        services::{GitOpsPlatform, VCS},
    },
    domain::repositories::ports::RepositoryConfigurationService,
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
    pub async fn from_github_event(id: String, event: WebhookEvent, payload: WebhookEventPayload, state: ApplicationState) -> Result<Option<Self>> {
        match payload {
            WebhookEventPayload::IssueComment(payload) => {
                let comment_body = if let Some(body) = payload.comment.body {
                    body
                } else {
                    // nothing to do, this isn't a comment we are interested in
                    return Ok(None);
                };

                // this is always Some
                let repository = event.repository.unwrap();
                let repository_configuration = match state.repository_configuration_service.get_repository_configuration(repository.url.as_ref()).await? {
                    Some(repository_configuration) => repository_configuration,
                    None => return Ok(None),
                };
                let octocrab = Octocrab::try_from(&repository_configuration)?;
                let vcs_service = VCS::GitHub(GitHub::from(octocrab));
                let last_commit_sha = vcs_service.fetch_pr_last_commit_sha(FetchPRLastCommitSHARequest {
                    repository_id: (*repository.id),
                    pr_number: payload.issue.number,
                }).await?;
                let refs_url = repository.git_refs_url.clone().unwrap();
                let config_url = refs_url.to_string().replace("{/sha}", &last_commit_sha);
                let configuration = vcs_service.download_file(DownloadFileRequest {
                    repository_id: (*repository.id),
                    path: config_url,
                    r#ref: last_commit_sha.clone(),
                    content_type: "application/yaml".to_string(),
                }).await?;

                let repository_full_name = repository.full_name.clone().unwrap();
                let (repository_owner, repository_name) = repository_full_name.split_once('/').unwrap();

                Ok(Some(Self {
                    id,
                    vcs: VcsContext {
                        comment: CommentContext {
                            body: comment_body,
                        },
                        repository: RepositoryContext::from(&repository),
                        pull_request: PullRequestContext {
                            number: payload.issue.number,
                            title: payload.issue.title.clone(),
                            last_commit_sha,
                        },
                    },
                    vcs_service: VCS::GitHub(crate::domain::environment::services::GitHub::from(octocrab::Octocrab::default())),
                    gitops_platform_service: GitOpsPlatform::ArgoCD(crate::domain::environment::services::ArgoCD),
                    config: SandcastleConfiguration {
                        custom: Value::Null,
                    },
                }))
            }
            _ => {
                Ok(None)
            }
        }
    }

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
    /// Whether the repository is private
    pub private: bool,
    /// The base URI of the repository
    pub url: String,
}

impl From<&Repository> for RepositoryContext {
    fn from(value: &Repository) -> Self {
        Self {
            name: value.name.clone(),
            private: value.private.unwrap_or(false),
            url: value.url.to_string(),
        }
    }
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

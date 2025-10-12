use std::{backtrace::Backtrace, str::FromStr};

use crate::{
    Result,
    application::ApplicationState,
    domain::environment::{
        models::{DownloadFileRequest, FetchPRLastCommitSHARequest},
        services::GitHub,
    },
    error::ServiceErrorCode,
};
use crate::{
    domain::environment::{
        models::config::{BuiltinConfigKey, ConfigPath, SandcastleConfiguration},
        ports::{Reconcile, VCSService},
        services::{GitOpsPlatform, VCS},
    },
    domain::repositories::ports::RepositoryConfigurationService,
    error::SandcastleError,
};
use octocrab::{
    Octocrab,
    models::{
        Repository,
        webhook_events::{WebhookEvent, WebhookEventPayload},
    },
};
use regex::Regex;
use serde_yaml::Value;

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
    pub fn template(&self, template: &str) -> Result<String, SandcastleError> {
        let mut result = template.to_string();
        let r = Regex::new(r#"\{\{ (.*?) \}\}"#).unwrap();
        let replacements: Vec<(String, String)> = r
            .captures_iter(&result)
            .map(|capture| -> Result<(String, String), SandcastleError> {
                let full_match = capture.get(0).unwrap().as_str().to_string();
                let path = capture.get(1).unwrap().as_str().trim();
                let value = self.get_config_value(path).ok_or_else(|| SandcastleError::Service {
                        code: ServiceErrorCode::InvalidConfiguration,
                        message: format!("Value not found for path: {}", path),
                        reason: path.to_string(),
                        backtrace: Backtrace::capture(),
                    })?;
                Ok((full_match, value))
            })
            .collect::<Result<Vec<_>, _>>()?;

        for (pattern, replacement) in replacements {
            result = result.replace(&pattern, &replacement);
        }
        Ok(result)
    }

    pub async fn from_github_event(
        id: String,
        event: WebhookEvent,
        payload: WebhookEventPayload,
        state: ApplicationState,
    ) -> Result<Option<Self>> {
        match payload {
            WebhookEventPayload::IssueComment(payload) => {
                // determine wether this is a comment we are interested in
                let comment_body = if let Some(body) = payload.comment.body {
                    body
                } else {
                    // nothing to do, this isn't a comment we are interested in
                    return Ok(None);
                };

                // this is always Some
                let repository = event.repository.unwrap();
                let repository_configuration = match state
                    .repository_configuration_service
                    .get_repository_configuration(repository.url.as_ref())
                    .await?
                {
                    Some(repository_configuration) => repository_configuration,
                    None => return Ok(None),
                };
                let octocrab = Octocrab::try_from(&repository_configuration)?;

                // Fetch config from repository
                let vcs_service = VCS::GitHub(GitHub::from(octocrab));
                let last_commit_sha = vcs_service
                    .fetch_pr_last_commit_sha(FetchPRLastCommitSHARequest {
                        repository_id: (*repository.id),
                        pr_number: payload.issue.number,
                    })
                    .await?;
                let refs_url = repository.git_refs_url.clone().unwrap();
                let config_url = refs_url.to_string().replace("{/sha}", &last_commit_sha);
                let configuration_file_content = vcs_service
                    .download_file(DownloadFileRequest {
                        repository_id: (*repository.id),
                        path: config_url,
                        r#ref: last_commit_sha.clone(),
                        content_type: "application/yaml".to_string(),
                    })
                    .await?;
                let config = SandcastleConfiguration::from_string(&configuration_file_content)?;

                Ok(Some(Self {
                    id,
                    vcs: VcsContext {
                        comment: CommentContext { body: comment_body },
                        repository: RepositoryContext::from(&repository),
                        pull_request: PullRequestContext {
                            number: payload.issue.number,
                            title: payload.issue.title.clone(),
                            last_commit_sha,
                        },
                    },
                    vcs_service: VCS::GitHub(crate::domain::environment::services::GitHub::from(
                        octocrab::Octocrab::default(),
                    )),
                    gitops_platform_service: GitOpsPlatform::ArgoCD(
                        crate::domain::environment::services::ArgoCD,
                    ),
                    config,
                }))
            }
            _ => Ok(None),
        }
    }

    fn get_config_value(&self, path: &str) -> Option<String> {
        if path.starts_with(".Sandcastle.") {
            self.get_builtin_config_value(path)
        } else {
            self.config.get_custom_value(path)
        }
    }

    fn get_builtin_config_value(&self, key: &str) -> Option<String> {
        let key = BuiltinConfigKey::from_key(key)?;
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

#[cfg(test)]
mod tests {
    use crate::domain::environment::services::ArgoCD;

    use super::*;

    async fn test_context() -> ReconcileContext {
        let context = ReconcileContext {
            id: "1".to_string(),
            vcs: VcsContext {
                repository: RepositoryContext {
                    name: "test".to_string(),
                    private: false,
                    url: "https://github.com/test/test".to_string(),
                },
                pull_request: PullRequestContext {
                    number: 1,
                    title: "test".to_string(),
                    last_commit_sha: "test".to_string(),
                },
                comment: CommentContext {
                    body: "test".to_string(),
                },
            },
            vcs_service: VCS::GitHub(GitHub::from(Octocrab::default())),
            gitops_platform_service: GitOpsPlatform::ArgoCD(ArgoCD),
            config: SandcastleConfiguration {
                custom: Value::Null,
            },
        };
        context
    }

    #[tokio::test]
    async fn test_template() {
        let template = "{{ .Sandcastle.EnvironmentName }}";
        let context = test_context().await;
        let result = context.template(template).unwrap();
        assert_eq!(result, "test");
    }
}

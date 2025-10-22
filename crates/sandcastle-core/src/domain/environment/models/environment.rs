use std::backtrace::Backtrace;

use crate::{
    Result,
    domain::environment::models::{DownloadFileRequest, FetchPRLastCommitSHARequest},
    error::ServiceErrorCode,
};
use crate::{
    domain::environment::{
        models::config::{BuiltinConfigKey, SandcastleConfiguration},
        ports::{Reconcile, VCSService},
        services::{GitOpsPlatform, Vcs},
    },
    error::SandcastleError,
};
use octocrab::models::{
    Repository,
    webhook_events::{WebhookEvent, WebhookEventPayload},
};
use regex::Regex;

#[derive(Clone)]
pub struct ReconcileContext {
    /// The ID of the reconcile context
    pub id: String,
    /// The VCS context
    pub vcs: VcsContext,
    /// The VCS service
    pub vcs_service: Vcs,
    /// The GitOps platform service
    pub gitops_platform_service: GitOpsPlatform,
    /// Sandcastle configuration
    pub config: SandcastleConfiguration,
    /// The trigger that initiated this reconciliation
    pub trigger: super::ReconcileTrigger,
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
                let value =
                    self.get_config_value(path)
                        .ok_or_else(|| SandcastleError::Service {
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
        vcs_service: Vcs,
        gitops_platform_service: GitOpsPlatform,
    ) -> Result<Option<Self>> {
        match payload {
            WebhookEventPayload::IssueComment(payload) => {
                let comment_body = if let Some(body) = payload.comment.body {
                    body
                } else {
                    return Ok(None);
                };

                let command = super::Command::parse(&comment_body)?;
                let trigger = match command {
                    Some(cmd) => super::ReconcileTrigger::CommentCommand(cmd),
                    None => return Ok(None),
                };

                let repository = event.repository.unwrap();

                let last_commit_sha = vcs_service
                    .fetch_pr_last_commit_sha(FetchPRLastCommitSHARequest {
                        repository_id: (*repository.id),
                        pr_number: payload.issue.number,
                    })
                    .await?;

                let config =
                    Self::fetch_config(&vcs_service, &repository, &last_commit_sha).await?;

                Ok(Some(Self::build(
                    id,
                    VcsContext {
                        comment: CommentContext { body: comment_body },
                        repository: RepositoryContext::from(&repository),
                        pull_request: PullRequestContext {
                            number: payload.issue.number,
                            title: payload.issue.title.clone(),
                            last_commit_sha,
                        },
                    },
                    vcs_service,
                    gitops_platform_service,
                    config,
                    trigger,
                )))
            }
            WebhookEventPayload::Push(payload) => {
                let repository = event.repository.unwrap();
                let commit_sha = payload.after;

                let config = Self::fetch_config(&vcs_service, &repository, &commit_sha).await?;

                Ok(Some(Self::build(
                    id,
                    VcsContext {
                        comment: CommentContext {
                            body: String::new(),
                        },
                        repository: RepositoryContext::from(&repository),
                        pull_request: PullRequestContext {
                            number: 0,
                            title: format!("Push to {}", payload.r#ref),
                            last_commit_sha: commit_sha,
                        },
                    },
                    vcs_service,
                    gitops_platform_service,
                    config,
                    super::ReconcileTrigger::PushEvent,
                )))
            }
            WebhookEventPayload::PullRequest(payload) => {
                if payload.action != octocrab::models::webhook_events::payload::PullRequestWebhookEventAction::Closed {
                    return Ok(None);
                }

                let repository = event.repository.unwrap();
                let pr = payload.pull_request;
                let last_commit_sha = pr.head.sha;

                let config =
                    Self::fetch_config(&vcs_service, &repository, &last_commit_sha).await?;

                Ok(Some(Self::build(
                    id,
                    VcsContext {
                        comment: CommentContext {
                            body: String::new(),
                        },
                        repository: RepositoryContext::from(&repository),
                        pull_request: PullRequestContext {
                            number: pr.number,
                            title: pr.title.unwrap_or_default(),
                            last_commit_sha,
                        },
                    },
                    vcs_service,
                    gitops_platform_service,
                    config,
                    super::ReconcileTrigger::PullRequestClosed,
                )))
            }
            _ => Ok(None),
        }
    }

    async fn fetch_config(
        vcs_service: &Vcs,
        repository: &Repository,
        commit_sha: &str,
    ) -> Result<SandcastleConfiguration> {
        let configuration_file_content = vcs_service
            .download_file(DownloadFileRequest {
                repository_id: (*repository.id),
                path: ".github/sandcastle.yaml".to_string(),
                r#ref: commit_sha.to_string(),
                content_type: "application/yaml".to_string(),
            })
            .await?;

        SandcastleConfiguration::from_string(&configuration_file_content)
    }

    fn build(
        id: String,
        vcs: VcsContext,
        vcs_service: Vcs,
        gitops_platform_service: GitOpsPlatform,
        config: SandcastleConfiguration,
        trigger: super::ReconcileTrigger,
    ) -> Self {
        Self {
            id,
            vcs,
            vcs_service,
            gitops_platform_service,
            config,
            trigger,
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
            BuiltinConfigKey::PRNumber => Some(self.vcs.pull_request.number.to_string()),
        }
    }

    pub fn labels(&self) -> Vec<(String, String)> {
        vec![
            (
                "environment".to_string(),
                self.config
                    .get_custom_value(".Sandcastle.EnvironmentName")
                    .unwrap_or_default(),
            ),
            ("repository".to_string(), self.vcs.repository.name.clone()),
            (
                "pull-request".to_string(),
                self.vcs.pull_request.number.to_string(),
            ),
        ]
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
            // this is always Some
            url: value.html_url.as_ref().unwrap().to_string(),
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
    pub applications: Vec<String>,
}

/// Action to delete an Argocd Application
#[derive(Debug, Clone)]
pub struct DeleteArgocdApplicationAction {
    /// The GitOps File
    pub applications: Vec<String>,
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
    use googletest::prelude::*;
    use octocrab::models::webhook_events::WebhookEventType;

    use crate::domain::environment::ports::{MockGitOpsPlatformService, MockVCSService};
    use googletest::Result;

    use super::*;

    async fn test_context() -> ReconcileContext {
        let config = SandcastleConfiguration::from_string(include_str!(
            "../../../../tests/fixtures/example_application_1.yaml"
        ))
        .unwrap();
        let context = ReconcileContext {
            id: "1".to_string(),
            vcs: VcsContext {
                repository: RepositoryContext {
                    name: "sandcastle-monorepo-test".to_string(),
                    private: false,
                    url: "https://github.com/mmoreiradj/sandcastle-monorepo-test".to_string(),
                },
                pull_request: PullRequestContext {
                    number: 1,
                    title: "feat: make unreasonable promises".to_string(),
                    last_commit_sha: "1234567890".to_string(),
                },
                comment: CommentContext {
                    body: "sandcastle deploy".to_string(),
                },
            },
            vcs_service: Vcs::MockVCS(MockVCSService::new()),
            gitops_platform_service: GitOpsPlatform::MockGitOpsPlatform(
                MockGitOpsPlatformService::new(),
            ),
            config: config,
            trigger: crate::domain::environment::models::ReconcileTrigger::CommentCommand(
                crate::domain::environment::models::Command::Deploy,
            ),
        };
        context
    }

    #[tokio::test]
    async fn test_small_template() {
        let template = "{{ .Sandcastle.EnvironmentName }}";
        let context = test_context().await;
        let result = context.template(template).unwrap();
        assert_eq!(result, "sandcastle-monorepo-test");
    }

    #[tokio::test]
    async fn test_large_template() {
        let template = include_str!("../../../../tests/fixtures/example_application_1.yaml");
        let context = test_context().await;
        let result = context.template(template).unwrap();
        insta::assert_snapshot!(result);
    }

    #[tokio::test]
    #[gtest]
    async fn test_from_github_event_issue_comment() -> Result<()> {
        let comment_payload_str =
            include_str!("../../../../tests/fixtures/github/issue_comment_webhook_event.json");
        let comment_payload =
            serde_json::from_str::<serde_json::Value>(comment_payload_str).unwrap();
        let event =
            WebhookEvent::try_from_header_and_body("issue_comment", comment_payload_str).unwrap();
        let webhook_event_type = WebhookEventType::IssueComment;
        let payload = webhook_event_type
            .parse_specific_payload(comment_payload)
            .unwrap();

        let mut mock_vcs_service = MockVCSService::new();
        mock_vcs_service
            .expect_fetch_pr_last_commit_sha()
            .returning(move |_| Ok("1234567890".to_string()));
        mock_vcs_service.expect_download_file().returning(move |_| {
            let template = include_str!("../../../../tests/fixtures/example_application_1.yaml");
            Ok(template.to_string())
        });
        let context = ReconcileContext::from_github_event(
            "1".to_string(),
            event,
            payload,
            Vcs::MockVCS(mock_vcs_service),
            GitOpsPlatform::MockGitOpsPlatform(MockGitOpsPlatformService::new()),
        )
        .await
        .unwrap()
        .unwrap();
        expect_that!(
            context.vcs.repository,
            matches_pattern!(RepositoryContext {
                name: eq("sandcastle-monorepo-test"),
                private: eq(&true),
                url: eq("https://github.com/mmoreiradj/sandcastle-monorepo-test"),
            })
        );
        expect_that!(
            context.vcs.pull_request,
            matches_pattern!(PullRequestContext {
                number: eq(&1),
                title: eq("feat: make unreasonable promises"),
                last_commit_sha: eq("1234567890"),
            })
        );
        expect_that!(
            context.vcs.comment,
            matches_pattern!(CommentContext {
                body: eq("sandcastle deploy"),
            })
        );
        Ok(())
    }
}

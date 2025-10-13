use tracing::{info, instrument};

use crate::{
    Result,
    domain::environment::models::{
        Command, CreateOrUpdateArgocdApplicationAction, DeleteArgocdApplicationAction,
        ReconcileActions, ReconcileContext, ReconcileTrigger,
    },
    error::SandcastleError,
};

pub struct ReconciliationService;

impl ReconciliationService {
    #[instrument(skip(context))]
    pub async fn reconcile(context: ReconcileContext) -> Result<(), SandcastleError> {
        info!("Processing reconciliation trigger: {:?}", context.trigger);

        let action = Self::determine_action(&context)?;

        info!("Executing action: {:?}", action);

        action.reconcile(context).await?;

        info!("Reconciliation completed successfully");

        Ok(())
    }

    fn determine_action(context: &ReconcileContext) -> Result<ReconcileActions, SandcastleError> {
        let templated_application = context.template(&context.config.template)?;

        match &context.trigger {
            ReconcileTrigger::CommentCommand(Command::Deploy) => {
                Ok(ReconcileActions::CreateOrUpdateArgocdApplication(
                    CreateOrUpdateArgocdApplicationAction {
                        application: templated_application,
                    },
                ))
            }
            ReconcileTrigger::CommentCommand(Command::Destroy) => Ok(
                ReconcileActions::DeleteArgocdApplication(DeleteArgocdApplicationAction {
                    application: templated_application,
                }),
            ),
            ReconcileTrigger::PushEvent => Ok(ReconcileActions::CreateOrUpdateArgocdApplication(
                CreateOrUpdateArgocdApplicationAction {
                    application: templated_application,
                },
            )),
            ReconcileTrigger::PullRequestClosed => Ok(ReconcileActions::DeleteArgocdApplication(
                DeleteArgocdApplicationAction {
                    application: templated_application,
                },
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::environment::{
        models::{
            CommentContext, PullRequestContext, RepositoryContext, config::SandcastleConfiguration,
            VcsContext,
        },
        ports::MockVCSService,
        services::{ArgoCD, GitOpsPlatform, VCS},
    };

    fn test_context(trigger: ReconcileTrigger) -> ReconcileContext {
        let config = SandcastleConfiguration::from_string(include_str!(
            "../../tests/fixtures/example_application_1.yaml"
        ))
        .unwrap();

        ReconcileContext {
            id: "test-123".to_string(),
            vcs: VcsContext {
                repository: RepositoryContext {
                    name: "test-repo".to_string(),
                    private: false,
                    url: "https://github.com/test/repo".to_string(),
                },
                pull_request: PullRequestContext {
                    number: 42,
                    title: "Test PR".to_string(),
                    last_commit_sha: "abc123".to_string(),
                },
                comment: CommentContext {
                    body: String::new(),
                },
            },
            vcs_service: VCS::MockVCS(MockVCSService::new()),
            gitops_platform_service: GitOpsPlatform::ArgoCD(ArgoCD),
            config,
            trigger,
        }
    }

    #[tokio::test]
    async fn test_reconcile_deploy_command() {
        let context = test_context(ReconcileTrigger::CommentCommand(Command::Deploy));
        let action = ReconciliationService::determine_action(&context).unwrap();

        match action {
            ReconcileActions::CreateOrUpdateArgocdApplication(action) => {
                assert!(action.application.contains("kind: Application"));
                assert!(action.application.contains("frontend-test-repo"));
            }
            _ => panic!("Expected CreateOrUpdateArgocdApplication action"),
        }
    }

    #[tokio::test]
    async fn test_reconcile_destroy_command() {
        let context = test_context(ReconcileTrigger::CommentCommand(Command::Destroy));
        let action = ReconciliationService::determine_action(&context).unwrap();

        match action {
            ReconcileActions::DeleteArgocdApplication(action) => {
                assert!(action.application.contains("kind: Application"));
            }
            _ => panic!("Expected DeleteArgocdApplication action"),
        }
    }

    #[tokio::test]
    async fn test_reconcile_push_event() {
        let context = test_context(ReconcileTrigger::PushEvent);
        let action = ReconciliationService::determine_action(&context).unwrap();

        match action {
            ReconcileActions::CreateOrUpdateArgocdApplication(action) => {
                assert!(action.application.contains("kind: Application"));
            }
            _ => panic!("Expected CreateOrUpdateArgocdApplication action"),
        }
    }

    #[tokio::test]
    async fn test_reconcile_pr_closed() {
        let context = test_context(ReconcileTrigger::PullRequestClosed);
        let action = ReconciliationService::determine_action(&context).unwrap();

        match action {
            ReconcileActions::DeleteArgocdApplication(action) => {
                assert!(action.application.contains("kind: Application"));
            }
            _ => panic!("Expected DeleteArgocdApplication action"),
        }
    }
}

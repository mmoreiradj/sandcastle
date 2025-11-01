use std::path::PathBuf;

use colored::Colorize;
use kube::{Client, Config};
use sandcastle_external_crds::argocd::application::Application;

use crate::{
    Result,
    domain::{
        environment::{
            models::{
                Command, CommentContext, PullRequestContext, ReconcileContext, ReconcileTrigger,
                RepositoryContext, VcsContext, config::SandcastleConfiguration,
            },
            ports::{MockGitOpsPlatformService, MockVCSService},
            services::{ArgoCD, GitOpsPlatform, Vcs},
        },
        repositories::{
            models::GitOpsPlatformType,
            services::{DefaultRepositoryConfigurationService, RepositoryConfigurations},
        },
    },
};

mod http;
mod operator;
pub(crate) mod reconciliation;

const ARGOCD_NAMESPACE: &str = "argocd";

#[derive(Debug, Clone)]
pub struct ApplicationConfig {
    pub argocd_namespace: String,
}

/// State shared beetween the HTTP and Operator
#[derive(Clone)]
pub(crate) struct ApplicationState {
    pub(crate) kube_client: Client,
    pub(crate) namespace: String,
    pub(crate) argocd_namespace: String,
    pub(crate) repository_configuration_service: RepositoryConfigurations,
}

impl ApplicationState {
    pub async fn new(application_config: ApplicationConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let kube_client = Client::try_default().await?;
        let config = Config::infer().await?;
        Ok(Self {
            kube_client,
            namespace: config.default_namespace,
            argocd_namespace: application_config.argocd_namespace,
            repository_configuration_service: DefaultRepositoryConfigurationService::default()
                .into(),
        })
    }

    pub fn operator_context(&self) -> operator::OperatorContext {
        operator::OperatorContext {
            client: self.kube_client.clone(),
            repository_configuration_service: self.repository_configuration_service.clone(),
            namespace: self.namespace.clone(),
        }
    }

    pub fn gitops_platform(&self, gitops_platform_type: &GitOpsPlatformType) -> GitOpsPlatform {
        match gitops_platform_type {
            GitOpsPlatformType::ArgoCD => GitOpsPlatform::ArgoCD(ArgoCD::new(
                self.kube_client.clone(),
                self.argocd_namespace.clone(),
            )),
        }
    }
}

pub async fn start(application_config: ApplicationConfig) -> Result<(), Box<dyn std::error::Error>> {
    let state = ApplicationState::new(application_config).await?;
    let context = state.operator_context();
    tokio::select! {
      _ = http::start(state.clone()) => {
        tracing::info!("HTTP server started");
        Ok(())
      }
      _ = operator::run(context) => {
        tracing::info!("Operator started");
        Ok(())
      }
    }
}

pub async fn test_application(
    file: PathBuf,
    gitops_platform_type: GitOpsPlatformType,
) -> Result<String, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(file)?;
    let config = SandcastleConfiguration::from_string(&content)?;
    let context = ReconcileContext {
        config: config.clone(),
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
                body: "test comment".to_string(),
            },
        },
        vcs_service: Vcs::MockVCS(MockVCSService::new()),
        gitops_platform_service: GitOpsPlatform::MockGitOpsPlatform(
            MockGitOpsPlatformService::new(),
        ),
        trigger: ReconcileTrigger::CommentCommand(Command::Deploy),
    };
    let applications = context.template(&config.template)?;
    match gitops_platform_type {
        GitOpsPlatformType::ArgoCD => Ok(applications
            .split("---")
            .map(|application_str| {
                let application_str = application_str.trim();
                let application: Result<Application, serde_yaml::Error> =
                    serde_yaml::from_str(application_str);
                match application {
                    Ok(_) => {
                        format!(
                            r#"{}
{application_str}
"#,
                            "# This application is valid".green(),
                        )
                    }
                    Err(e) => {
                        format!(
                            r#"{}
{application_str}
"#,
                            format!("# This application is invalid: {e:#?}").red(),
                        )
                    }
                }
            })
            .collect::<String>()),
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use super::*;

    #[tokio::test]
    async fn test_test_application() {
        let file = PathBuf::from("tests/fixtures/example_application_2.yaml");
        let applications = test_application(file, GitOpsPlatformType::ArgoCD)
            .await
            .unwrap();
        assert_snapshot!(applications);
    }
}

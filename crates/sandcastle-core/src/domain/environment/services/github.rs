use std::backtrace::Backtrace;

use async_trait::async_trait;
use octocrab::Octocrab;

use crate::{
    domain::environment::{
        models::{DownloadFileRequest, FetchPRLastCommitSHARequest},
        ports::VCSService,
    },
    error::{SandcastleError, ServiceErrorCode},
};

#[derive(Debug, Clone)]
pub struct GitHub {
    client: Octocrab,
}

impl From<Octocrab> for GitHub {
    fn from(client: Octocrab) -> Self {
        Self { client }
    }
}

#[async_trait]
impl VCSService for GitHub {
    async fn download_file(&self, request: DownloadFileRequest) -> Result<String, SandcastleError> {
        tracing::info!("Downloading file {} from GitHub", request.path);
        let mut file = self
            .client
            .repos_by_id(request.repository_id)
            .get_content()
            .path(request.path.clone())
            .r#ref(request.r#ref)
            .send()
            .await
            .map_err(|e| SandcastleError::Service {
                code: ServiceErrorCode::VCSFileDownloadFailed,
                message: e.to_string(),
                reason: request.path.clone(),
                backtrace: Backtrace::capture(),
            })?;

        let file = file
            .take_items()
            .first()
            .ok_or(SandcastleError::Service {
                code: ServiceErrorCode::VCSFileNotFound,
                message: "File not found".to_string(),
                reason: request.path.clone(),
                backtrace: Backtrace::capture(),
            })?
            .decoded_content()
            .ok_or(SandcastleError::Service {
                code: ServiceErrorCode::VCSFileDownloadFailed,
                message: "Failure to decode file content".to_string(),
                reason: request.path.clone(),
                backtrace: Backtrace::capture(),
            })?
            .to_string();

        Ok(file)
    }

    async fn fetch_pr_last_commit_sha(
        &self,
        request: FetchPRLastCommitSHARequest,
    ) -> Result<String, SandcastleError> {
        tracing::info!(
            "Fetching last commit SHA from GitHub for PR {}",
            request.pr_number
        );
        let repository = self
            .client
            .repos_by_id(request.repository_id)
            .get()
            .await
            .map_err(|e| SandcastleError::Service {
                code: ServiceErrorCode::VCSFetchPRLastCommitSHARequest,
                message: e.to_string(),
                reason: request.repository_id.to_string(),
                backtrace: Backtrace::capture(),
            })?;
        let pr = self
            .client
            .pulls(repository.owner.unwrap().login, repository.name)
            .get(request.pr_number)
            .await
            .map_err(|e| SandcastleError::Service {
                code: ServiceErrorCode::VCSFetchPRLastCommitSHARequest,
                message: e.to_string(),
                reason: format!("{:?}", e),
                backtrace: Backtrace::capture(),
            })?;
        Ok(pr.head.sha)
    }
}

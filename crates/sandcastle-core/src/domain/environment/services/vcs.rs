use std::backtrace::Backtrace;

use async_trait::async_trait;
use octocrab::Octocrab;
use tracing::instrument;

use crate::{
    domain::{environment::models::DownloadFileRequest, environment::ports::VCSService},
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
    #[instrument(skip(self))]
    async fn download_file(
        &self,
        request: DownloadFileRequest,
    ) -> Result<Vec<u8>, SandcastleError> {
        let file = self
            .client
            .download(&request.uri, &request.content_type)
            .await
            .map_err(|e| SandcastleError::Service {
                code: ServiceErrorCode::VCSFileDownloadFailed,
                message: e.to_string(),
                reason: request.uri.clone(),
                backtrace: Backtrace::capture(),
            })?;
        Ok(file)
    }
}

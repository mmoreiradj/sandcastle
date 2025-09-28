use std::backtrace::Backtrace;

use octocrab::Octocrab;
use tracing::instrument;

use crate::{domain::vcs::ports::VCService, error::{SandcastleError, ServiceErrorCode}};

#[derive(Debug, Clone)]
pub struct GitHubVCS {
    client: Octocrab,
}

impl From<Octocrab> for GitHubVCS {
    fn from(client: Octocrab) -> Self {
        Self { client }
    }
}

impl VCService for GitHubVCS {
    #[instrument(skip(self))]
    async fn download_file(
        &self,
        request: super::ports::DownloadFileRequest,
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

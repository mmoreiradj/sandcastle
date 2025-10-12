use std::backtrace::Backtrace;

use octocrab::Octocrab;
use snafu::ResultExt;

use crate::error::{SandcastleError, ServiceErrorCode};

#[derive(Debug, Clone)]
pub struct RepositoryConfiguration {
    pub repository_url: String,
    pub authentication: Authentication,
}

impl TryFrom<&RepositoryConfiguration> for Octocrab {
    type Error = SandcastleError;

    fn try_from(value: &RepositoryConfiguration) -> Result<Self, Self::Error> {
        match value.authentication.clone() {
            Authentication::GitHubApp(auth) => {
                let key = jsonwebtoken::EncodingKey::from_rsa_pem(auth.private_key.as_bytes())
                    .whatever_context(format!(
                        "Failed to encode private key for GitHub app {}",
                        auth.app_id
                    ))?;
                let octocrab = Octocrab::builder()
                    .app(auth.app_id.into(), key)
                    .build()
                    .map_err(|e| SandcastleError::Service {
                        code: ServiceErrorCode::GitHubAppAuthentication,
                        message: e.to_string(),
                        reason: auth.app_id.to_string(),
                        backtrace: Backtrace::capture(),
                    })?;
                let octocrab = octocrab
                    .installation(auth.installation_id.into())
                    .map_err(|e| SandcastleError::Service {
                        code: ServiceErrorCode::GitHubAppAuthentication,
                        message: e.to_string(),
                        reason: auth.app_id.to_string(),
                        backtrace: Backtrace::capture(),
                    })?;
                Ok(octocrab)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Authentication {
    GitHubApp(GitHubAppAuthentication),
}

#[derive(Debug, Clone)]
pub struct GitHubAppAuthentication {
    pub app_id: u64,
    pub installation_id: u64,
    pub private_key: String,
}

use std::{backtrace::Backtrace, fmt::Display};

use validator::ValidationErrors;

#[derive(Clone, Debug)]
pub enum ServiceErrorCode {
    HelmRepoAddFailed,
    HelmRepoIndexFailed,
    HelmChartNotFound,
    HelmChartVersionNotFound,
    HelmChartDownloadFailed,
    HelmInstallOrUpgradeFailed,
    HelmUninstallFailed,
    HelmReleaseStatusFailed,
    VCSFileDownloadFailed,
    VCSFileNotFound,
    InvalidConfiguration,
    SecretParsingFailed,
    VCSFetchPRLastCommitSHARequest,
    GitHubAppAuthentication,
}

impl Display for ServiceErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceErrorCode::HelmRepoAddFailed => write!(f, "helm_repo_add_failed"),
            ServiceErrorCode::HelmRepoIndexFailed => write!(f, "helm_repo_index_failed"),
            ServiceErrorCode::HelmChartNotFound => write!(f, "helm_chart_not_found"),
            ServiceErrorCode::HelmChartVersionNotFound => write!(f, "helm_chart_version_not_found"),
            ServiceErrorCode::HelmChartDownloadFailed => write!(f, "helm_chart_download_failed"),
            ServiceErrorCode::HelmInstallOrUpgradeFailed => {
                write!(f, "helm_install_or_upgrade_failed")
            }
            ServiceErrorCode::HelmUninstallFailed => write!(f, "helm_uninstall_failed"),
            ServiceErrorCode::HelmReleaseStatusFailed => write!(f, "helm_release_status_failed"),
            ServiceErrorCode::VCSFileDownloadFailed => write!(f, "vcs_file_download_failed"),
            ServiceErrorCode::VCSFileNotFound => write!(f, "vcs_file_not_found"),
            ServiceErrorCode::VCSFetchPRLastCommitSHARequest => {
                write!(f, "vcs_fetch_pr_last_commit_sha_request")
            }
            ServiceErrorCode::InvalidConfiguration => write!(f, "invalid_configuration"),
            ServiceErrorCode::SecretParsingFailed => write!(f, "secret_parsing_failed"),
            ServiceErrorCode::GitHubAppAuthentication => write!(f, "github_app_authentication"),
        }
    }
}

#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub))]
pub enum SandcastleError {
    #[snafu(display("{}: {}", message, source))]
    Serde {
        message: String,
        source: serde_json::Error,
        backtrace: Backtrace,
    },
    #[snafu(display("{}: {}", message, source))]
    Validation {
        message: String,
        source: ValidationErrors,
        backtrace: Backtrace,
    },
    #[snafu(display("{}: {}", code, message))]
    Service {
        code: ServiceErrorCode,
        message: String,
        reason: String,
        backtrace: Backtrace,
    },
    #[snafu(whatever, display("{message}: {source:?}"))]
    Unexpected {
        message: String,
        #[snafu(source(from(Box<dyn std::error::Error + Send + Sync>, Some)))]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        backtrace: Backtrace,
    },
    #[snafu(display("{}: {}", message, source))]
    Kube {
        message: String,
        source: kube::Error,
        backtrace: Backtrace,
    },
    #[snafu(display("Finalizer error: {source}"))]
    Finalizer {
        #[snafu(source(from(kube::runtime::finalizer::Error<SandcastleError>, Box::new)))]
        source: Box<kube::runtime::finalizer::Error<SandcastleError>>,
        backtrace: Backtrace,
    },
}

use std::{backtrace::Backtrace, fmt::Display};

use validator::ValidationErrors;

#[derive(Clone, Debug)]
pub enum ServiceErrorCode {
    HelmRepoAddFailed,
    HelmRepoIndexFailed,
    HelmChartNotFound,
    HelmChartVersionNotFound,
    HelmInstallOrUpgradeFailed,
    HelmUninstallFailed,
}

impl Display for ServiceErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceErrorCode::HelmRepoAddFailed => write!(f, "helm_repo_add_failed"),
            ServiceErrorCode::HelmRepoIndexFailed => write!(f, "helm_repo_index_failed"),
            ServiceErrorCode::HelmChartNotFound => write!(f, "helm_chart_not_found"),
            ServiceErrorCode::HelmChartVersionNotFound => write!(f, "helm_chart_version_not_found"),
            ServiceErrorCode::HelmInstallOrUpgradeFailed => {
                write!(f, "helm_install_or_upgrade_failed")
            }
            ServiceErrorCode::HelmUninstallFailed => write!(f, "helm_uninstall_failed"),
        }
    }
}

#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub))]
pub enum SandcastleProjectError {
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
}

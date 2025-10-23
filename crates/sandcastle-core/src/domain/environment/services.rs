mod environment;
mod github;
mod gitops;

use enum_dispatch::enum_dispatch;

pub use github::*;
pub use gitops::*;
use octocrab::Octocrab;

use crate::domain::environment::models::*;
#[cfg(test)]
use crate::domain::environment::ports::MockGitOpsPlatformService as MockGitOpsPlatform;
#[cfg(test)]
use crate::domain::environment::ports::MockVCSService as MockVCS;
use crate::domain::environment::ports::*;
use crate::domain::repositories::models::Authentication;
use crate::domain::repositories::models::RepositoryConfiguration;
use crate::error::SandcastleError;

#[enum_dispatch(VCSService)]
#[derive(Clone)]
pub enum Vcs {
    GitHub,
    #[cfg(test)]
    MockVCS,
}

impl TryFrom<&RepositoryConfiguration> for Vcs {
    type Error = SandcastleError;

    fn try_from(value: &RepositoryConfiguration) -> Result<Self, Self::Error> {
        match &value.authentication {
            Authentication::GitHubApp(_) => {
                let octocrab = Octocrab::try_from(value)?;
                Ok(Vcs::GitHub(GitHub::from(octocrab)))
            }
        }
    }
}

#[enum_dispatch(GitOpsPlatformService)]
#[derive(Clone)]
pub enum GitOpsPlatform {
    ArgoCD,
    #[cfg(test)]
    MockGitOpsPlatform,
}

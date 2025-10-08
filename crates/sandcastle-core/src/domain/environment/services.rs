mod environment;
mod gitops;
mod vcs;

use enum_dispatch::enum_dispatch;

pub use gitops::*;
pub use vcs::*;

use crate::domain::environment::models::*;
use crate::domain::environment::ports::*;
use crate::error::SandcastleError;

#[enum_dispatch(VCSService)]
#[derive(Clone)]
pub enum VCS {
    GitHub,
}

#[enum_dispatch(GitOpsPlatformService)]
#[derive(Clone)]
pub enum GitOpsPlatform {
    ArgoCD,
}

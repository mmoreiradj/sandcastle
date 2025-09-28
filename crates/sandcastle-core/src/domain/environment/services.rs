mod environment;
mod vcs;
mod gitops;

use enum_dispatch::enum_dispatch;

pub use environment::*;
pub use vcs::*;
pub use gitops::*;

use crate::domain::environment::ports::*;
use crate::domain::environment::models::*;
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

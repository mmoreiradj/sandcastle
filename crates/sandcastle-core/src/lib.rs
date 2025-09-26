use crate::error::SandcastleError;

pub mod crd;
pub mod error;
pub mod operator;
pub(crate) type Result<T, E = SandcastleError> = std::result::Result<T, E>;

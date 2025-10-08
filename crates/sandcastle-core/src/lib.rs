use crate::error::SandcastleError;

pub mod application;
mod domain;
mod error;
mod infrastructure;

pub type Result<T, E = SandcastleError> = std::result::Result<T, E>;

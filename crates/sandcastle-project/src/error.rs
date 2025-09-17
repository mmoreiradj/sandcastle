use std::backtrace::Backtrace;

use validator::ValidationErrors;

#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub))]
pub enum SandcastleProjectError {
    #[snafu(display("{}: {}", message, source))]
    Validation {
        message: String,
        source: ValidationErrors,
        backtrace: Backtrace,
    },
}

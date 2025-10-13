use std::backtrace::Backtrace;

use crate::error::{SandcastleError, ServiceErrorCode};

#[derive(Debug, Clone, PartialEq)]
pub enum ReconcileTrigger {
    CommentCommand(Command),
    PushEvent,
    PullRequestClosed,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Deploy,
    Destroy,
}

impl Command {
    pub fn parse(comment_body: &str) -> Result<Option<Self>, SandcastleError> {
        let trimmed = comment_body.trim().to_lowercase();

        if trimmed.starts_with("sandcastle deploy") || trimmed == "sandcastle deploy" {
            Ok(Some(Command::Deploy))
        } else if trimmed.starts_with("sandcastle destroy") || trimmed == "sandcastle destroy" {
            Ok(Some(Command::Destroy))
        } else if trimmed.starts_with("sandcastle") {
            Err(SandcastleError::Service {
                code: ServiceErrorCode::InvalidConfiguration,
                message: format!("Unknown sandcastle command: {}", comment_body),
                reason: "Expected 'sandcastle deploy' or 'sandcastle destroy'".to_string(),
                backtrace: Backtrace::capture(),
            })
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_deploy() {
        let result = Command::parse("sandcastle deploy").unwrap();
        assert_eq!(result, Some(Command::Deploy));

        let result = Command::parse("  SANDCASTLE DEPLOY  ").unwrap();
        assert_eq!(result, Some(Command::Deploy));

        let result = Command::parse("sandcastle deploy with extra args").unwrap();
        assert_eq!(result, Some(Command::Deploy));
    }

    #[test]
    fn test_parse_destroy() {
        let result = Command::parse("sandcastle destroy").unwrap();
        assert_eq!(result, Some(Command::Destroy));

        let result = Command::parse("  SANDCASTLE DESTROY  ").unwrap();
        assert_eq!(result, Some(Command::Destroy));
    }

    #[test]
    fn test_parse_non_sandcastle_comment() {
        let result = Command::parse("This is just a regular comment").unwrap();
        assert_eq!(result, None);

        let result = Command::parse("LGTM!").unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_invalid_sandcastle_command() {
        let result = Command::parse("sandcastle unknown");
        assert!(result.is_err());
    }
}

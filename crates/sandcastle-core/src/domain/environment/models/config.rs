use std::{backtrace::Backtrace, ops::Deref, str::FromStr, sync::OnceLock};

use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::error::{SandcastleError, ServiceErrorCode};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BuiltinConfigKey {
    EnvironmentName,
    RepoURL,
    TargetRevision,
    LastCommitSHA,
    PRNumber,
}

impl BuiltinConfigKey {
    pub fn from_key(key: &str) -> Option<Self> {
        match key {
            ".Sandcastle.EnvironmentName" => Some(BuiltinConfigKey::EnvironmentName),
            ".Sandcastle.RepoURL" => Some(BuiltinConfigKey::RepoURL),
            ".Sandcastle.TargetRevision" => Some(BuiltinConfigKey::TargetRevision),
            ".Sandcastle.LastCommitSHA" => Some(BuiltinConfigKey::LastCommitSHA),
            ".Sandcastle.PRNumber" => Some(BuiltinConfigKey::PRNumber),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConfigPath(String);

impl Deref for ConfigPath {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

static VALIDATION_REGEX: OnceLock<Regex> = OnceLock::new();

fn get_regex() -> &'static Regex {
    VALIDATION_REGEX.get_or_init(|| {
        Regex::new(r#"^\.(?:Sandcastle|Custom)(?:\.[A-Za-z0-9]+)+$"#)
            .expect("Failed to compile regex")
    })
}

impl FromStr for ConfigPath {
    type Err = SandcastleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !get_regex().is_match(s) {
            return Err(SandcastleError::Service {
                code: ServiceErrorCode::InvalidConfiguration,
                message: "Invalid config path, must match .<Sandcastle|Custom>.<key>. Ex: .Sandcastle.EnvironmentName or .Custom.baseDomain".to_string(),
                reason: s.to_string(),
                backtrace: Backtrace::capture(),
            });
        }

        Ok(ConfigPath(s.to_string()))
    }
}

/// Represents the sandcastle configuration from the application file
/// found in the repository.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandcastleConfiguration {
    pub custom: SandcastleCustomValues,
    pub template: String,
}

pub type SandcastleCustomValues = Value;

impl SandcastleConfiguration {
    pub fn from_string(string: &str) -> Result<Self, SandcastleError> {
        match string.trim().trim_start_matches("---").split_once("---") {
            Some((config, template)) => {
                let custom_config = serde_yaml::from_str::<Value>(config)
                    .map_err(|e| SandcastleError::Service {
                        code: ServiceErrorCode::InvalidConfiguration,
                        message: e.to_string(),
                        reason: config.to_string(),
                        backtrace: Backtrace::capture(),
                    })?
                    .get("custom")
                    .unwrap_or(&Value::Null)
                    .clone();
                Ok(Self {
                    custom: custom_config,
                    template: template.to_string(),
                })
            }
            None => Err(SandcastleError::Service {
                code: ServiceErrorCode::InvalidConfiguration,
                message: "No configuration found in file".to_string(),
                reason: string.to_string(),
                backtrace: Backtrace::capture(),
            }),
        }
    }

    pub fn get_custom_value(&self, path: &str) -> Option<String> {
        let path_parts = path
            .trim_start_matches(".Custom.")
            .split(".")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut current = &self.custom;

        for part in path_parts {
            println!("Part: {}", part);
            current = current.get(part.as_str())?;
        }

        current.as_str().map(|s| s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_custom_value() {
        let custom = r#"
        custom:
          baseDomain: sandcastle.dev
          whatever:
            key: value
        ---
        whatever:
          key: value
        "#;
        let config = SandcastleConfiguration::from_string(custom).unwrap();
        let value = config.get_custom_value(".Custom.baseDomain");
        assert_eq!(value, Some("sandcastle.dev".to_string()));

        let value = config.get_custom_value(".Custom.baseDomain.subDomain");
        assert_eq!(value, None);

        let value = config.get_custom_value(".Custom.baseDomain.subDomain.subDomain");
        assert_eq!(value, None);

        let value = config.get_custom_value(".Custom.whatever.key");
        assert_eq!(value, Some("value".to_string()));
    }

    #[test]
    fn test_from_string() {
        let application_yaml =
            include_str!("../../../../tests/fixtures/example_application_1.yaml");
        let config = SandcastleConfiguration::from_string(application_yaml).unwrap();
        assert_eq!(
            config.get_custom_value(".Custom.baseDomain"),
            Some("sandcastle.dev".to_string())
        );
        assert_eq!(
            config.get_custom_value(".Custom.whatever.key"),
            Some("value".to_string())
        );
    }
}

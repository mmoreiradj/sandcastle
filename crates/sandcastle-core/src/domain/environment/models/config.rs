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
}

impl BuiltinConfigKey {
    pub fn from_key(key: &str) -> Option<Self> {
        match key {
            ".Sandcastle.EnvironmentName" => Some(BuiltinConfigKey::EnvironmentName),
            ".Sandcastle.RepoURL" => Some(BuiltinConfigKey::RepoURL),
            ".Sandcastle.TargetRevision" => Some(BuiltinConfigKey::TargetRevision),
            ".Sandcastle.LastCommitSHA" => Some(BuiltinConfigKey::LastCommitSHA),
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
    pub application: String,
}

pub type SandcastleCustomValues = Value;

impl SandcastleConfiguration {
    pub fn from_string(string: &str) -> Result<Self, SandcastleError> {
        let parts = string
            .trim()
            .split("---")
            .filter_map(|s| if !s.is_empty() { Some(s.trim()) } else { None })
            .collect::<Vec<&str>>();

        match parts.first() {
            Some(part) => {
                let config = Self::from_yaml(part)?;
                Ok(config)
            }
            None => Err(SandcastleError::Service {
                code: ServiceErrorCode::InvalidConfiguration,
                message: "No configuration found in file".to_string(),
                reason: string.to_string(),
                backtrace: Backtrace::capture(),
            }),
        }
    }

    fn from_yaml(yaml: &str) -> Result<Self, SandcastleError> {
        println!("From YAML: {}", yaml);
        let config: SandcastleConfiguration =
            serde_yaml::from_str(yaml).map_err(|e| SandcastleError::Service {
                code: ServiceErrorCode::InvalidConfiguration,
                message: e.to_string(),
                reason: yaml.to_string(),
                backtrace: Backtrace::capture(),
            })?;
        Ok(config)
    }

    pub fn get_custom_value(&self, path: &str) -> Option<String> {
        println!("Getting custom value for path: {}", path);
        let path_parts = path
            .trim_start_matches(".Custom.")
            .split(".")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut current = &self.custom;

        for part in path_parts {
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
        "#;
        let config = SandcastleConfiguration::from_yaml(custom).unwrap();
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

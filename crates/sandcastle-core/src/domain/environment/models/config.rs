use std::backtrace::Backtrace;

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
            "EnvironmentName" => Some(BuiltinConfigKey::EnvironmentName),
            "RepoURL" => Some(BuiltinConfigKey::RepoURL),
            "TargetRevision" => Some(BuiltinConfigKey::TargetRevision),
            "LastCommitSHA" => Some(BuiltinConfigKey::LastCommitSHA),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConfigPath {
    Builtin(BuiltinConfigKey),
    Custom(Vec<String>),
}

impl ConfigPath {
    pub fn parse(path: &str) -> Option<Self> {
        let parts: Vec<&str> = path.split('.').filter(|s| !s.is_empty()).collect();

        if parts.first() != Some(&"Sandcastle") {
            return None;
        }

        match parts.get(1) {
            Some(&"Custom") if parts.len() > 2 => {
                let custom_parts = parts[2..].iter().map(|s| s.to_string()).collect();
                Some(ConfigPath::Custom(custom_parts))
            }
            Some(key) if parts.len() == 2 => {
                BuiltinConfigKey::from_key(key).map(ConfigPath::Builtin)
            }
            _ => None,
        }
    }
}

/// Represents the sandcastle configuration from the application file
/// found in the repository.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandcastleConfiguration {
    pub custom: SandcastleCustomValues,
}

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
            None => {
                return Err(SandcastleError::Service {
                    code: ServiceErrorCode::InvalidConfiguration,
                    message: "No configuration found in file".to_string(),
                    reason: string.to_string(),
                    backtrace: Backtrace::capture(),
                });
            }
        }
    }

    fn from_yaml(yaml: &str) -> Result<Self, SandcastleError> {
        let config: SandcastleConfiguration =
            serde_yaml::from_str(yaml).map_err(|e| SandcastleError::Service {
                code: ServiceErrorCode::InvalidConfiguration,
                message: e.to_string(),
                reason: yaml.to_string(),
                backtrace: Backtrace::capture(),
            })?;
        Ok(config)
    }

    pub fn get_custom_value(&self, path_parts: &[String]) -> Option<String> {
        let mut current = &self.custom;

        for part in path_parts {
            current = current.get(part.as_str())?;
        }

        current.as_str().map(|s| s.to_string())
    }
}

pub type SandcastleCustomValues = Value;

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
        let value = config.get_custom_value(&["baseDomain".to_string()]);
        assert_eq!(value, Some("sandcastle.dev".to_string()));

        let value = config.get_custom_value(&["baseDomain".to_string(), "subDomain".to_string()]);
        assert_eq!(value, None);

        let value = config.get_custom_value(&[
            "baseDomain".to_string(),
            "subDomain".to_string(),
            "subDomain".to_string(),
        ]);
        assert_eq!(value, None);

        let value = config.get_custom_value(&["whatever".to_string(), "key".to_string()]);
        assert_eq!(value, Some("value".to_string()));
    }

    #[test]
    fn test_from_string() {
        let application_yaml = include_str!("../../../../tests/fixtures/example_application_1.yaml");
        let config = SandcastleConfiguration::from_string(application_yaml);
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.get_custom_value(&["baseDomain".to_string()]), Some("sandcastle.dev".to_string()));
        assert_eq!(config.get_custom_value(&["whatever".to_string(), "key".to_string()]), Some("value".to_string()));
    }
}

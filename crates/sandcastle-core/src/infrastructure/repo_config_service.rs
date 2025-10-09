use std::{collections::BTreeMap, sync::Arc};

use k8s_openapi::api::core::v1::Secret;

use crate::{
    domain::repositories::models::{
        Authentication, GitHubAppAuthentication, RepositoryConfiguration,
    },
    error::{SandcastleError, ServiceErrorCode},
};

#[derive(Clone)]
pub struct GithubAppSecretData {
    pub url: String,
    pub app_id: u64,
    pub app_installation_id: u64,
    pub private_key: String,
}

impl From<GithubAppSecretData> for RepositoryConfiguration {
    fn from(secret: GithubAppSecretData) -> Self {
        Self {
            repository_url: secret.url,
            authentication: Authentication::GitHubApp(GitHubAppAuthentication {
                app_id: secret.app_id,
                installation_id: secret.app_installation_id,
                private_key: secret.private_key,
            }),
        }
    }
}

impl GithubAppSecretData {
    pub fn from_secret(secret: Arc<Secret>) -> Result<Self, SandcastleError> {
        let mut merged_data = BTreeMap::new();

        if let Some(data) = secret.data.clone() {
            for (key, value) in data {
                let decoded = String::from_utf8(value.0).map_err(|e| SandcastleError::Service {
                    code: ServiceErrorCode::SecretParsingFailed,
                    message: format!("Failed to decode base64 data for key: {}", key),
                    reason: e.to_string(),
                    backtrace: std::backtrace::Backtrace::capture(),
                })?;
                merged_data.insert(key, decoded);
            }
        }

        if let Some(string_data) = secret.string_data.clone() {
            for (key, value) in string_data {
                merged_data.insert(key, value);
            }
        }

        let url = merged_data
            .get("url")
            .ok_or_else(|| SandcastleError::Service {
                code: ServiceErrorCode::SecretParsingFailed,
                message: "Missing 'url' field in secret".to_string(),
                reason: "Required field 'url' not found in secret data".to_string(),
                backtrace: std::backtrace::Backtrace::capture(),
            })?
            .clone();

        let app_id = merged_data
            .get("app_id")
            .ok_or_else(|| SandcastleError::Service {
                code: ServiceErrorCode::SecretParsingFailed,
                message: "Missing 'app_id' field in secret".to_string(),
                reason: "Required field 'app_id' not found in secret data".to_string(),
                backtrace: std::backtrace::Backtrace::capture(),
            })?
            .parse::<u64>()
            .map_err(|e| SandcastleError::Service {
                code: ServiceErrorCode::SecretParsingFailed,
                message: "Failed to parse 'app_id' as u64".to_string(),
                reason: e.to_string(),
                backtrace: std::backtrace::Backtrace::capture(),
            })?;

        let app_installation_id = merged_data
            .get("app_installation_id")
            .ok_or_else(|| SandcastleError::Service {
                code: ServiceErrorCode::SecretParsingFailed,
                message: "Missing 'app_installation_id' field in secret".to_string(),
                reason: "Required field 'app_installation_id' not found in secret data".to_string(),
                backtrace: std::backtrace::Backtrace::capture(),
            })?
            .parse::<u64>()
            .map_err(|e| SandcastleError::Service {
                code: ServiceErrorCode::SecretParsingFailed,
                message: "Failed to parse 'app_installation_id' as u64".to_string(),
                reason: e.to_string(),
                backtrace: std::backtrace::Backtrace::capture(),
            })?;

        let private_key = merged_data
            .get("private_key")
            .ok_or_else(|| SandcastleError::Service {
                code: ServiceErrorCode::SecretParsingFailed,
                message: "Missing 'private_key' field in secret".to_string(),
                reason: "Required field 'private_key' not found in secret data".to_string(),
                backtrace: std::backtrace::Backtrace::capture(),
            })?
            .clone();

        Ok(Self {
            url,
            app_id,
            app_installation_id,
            private_key,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_openapi::{ByteString, api::core::v1::Secret};
    use std::collections::BTreeMap;

    #[test]
    fn test_from_secret_only_data() {
        let mut data = BTreeMap::new();
        data.insert(
            "url".to_string(),
            ByteString("https://github.com/test/repo.git".as_bytes().to_vec()),
        );
        data.insert(
            "app_id".to_string(),
            ByteString("12345".as_bytes().to_vec()),
        );
        data.insert(
            "app_installation_id".to_string(),
            ByteString("67890".as_bytes().to_vec()),
        );
        data.insert(
            "private_key".to_string(),
            ByteString(
                "-----BEGIN RSA PRIVATE KEY-----\ntest_key\n-----END RSA PRIVATE KEY-----"
                    .as_bytes()
                    .to_vec(),
            ),
        );

        let secret = Secret {
            data: Some(data),
            string_data: None,
            ..Default::default()
        };

        let result = GithubAppSecretData::from_secret(Arc::new(secret));
        assert!(result.is_ok());

        let auth = result.unwrap();
        assert_eq!(auth.url, "https://github.com/test/repo.git");
        assert_eq!(auth.app_id, 12345);
        assert_eq!(auth.app_installation_id, 67890);
        assert_eq!(
            auth.private_key,
            "-----BEGIN RSA PRIVATE KEY-----\ntest_key\n-----END RSA PRIVATE KEY-----"
        );
    }

    #[test]
    fn test_from_secret_only_string_data() {
        let mut string_data = BTreeMap::new();
        string_data.insert(
            "url".to_string(),
            "https://github.com/test/repo.git".to_string(),
        );
        string_data.insert("app_id".to_string(), "12345".to_string());
        string_data.insert("app_installation_id".to_string(), "67890".to_string());
        string_data.insert(
            "private_key".to_string(),
            "-----BEGIN RSA PRIVATE KEY-----\ntest_key\n-----END RSA PRIVATE KEY-----".to_string(),
        );

        let secret = Secret {
            data: None,
            string_data: Some(string_data),
            ..Default::default()
        };

        let result = GithubAppSecretData::from_secret(Arc::new(secret));
        assert!(result.is_ok());

        let auth = result.unwrap();
        assert_eq!(auth.url, "https://github.com/test/repo.git");
        assert_eq!(auth.app_id, 12345);
        assert_eq!(auth.app_installation_id, 67890);
        assert_eq!(
            auth.private_key,
            "-----BEGIN RSA PRIVATE KEY-----\ntest_key\n-----END RSA PRIVATE KEY-----"
        );
    }

    #[test]
    fn test_from_secret_mixed_data() {
        let mut data = BTreeMap::new();
        data.insert(
            "private_key".to_string(),
            ByteString(
                "-----BEGIN RSA PRIVATE KEY-----\ntest_key\n-----END RSA PRIVATE KEY-----"
                    .as_bytes()
                    .to_vec(),
            ),
        );

        let mut string_data = BTreeMap::new();
        string_data.insert(
            "url".to_string(),
            "https://github.com/test/repo.git".to_string(),
        );
        string_data.insert("app_id".to_string(), "12345".to_string());
        string_data.insert("app_installation_id".to_string(), "67890".to_string());

        let secret = Secret {
            data: Some(data),
            string_data: Some(string_data),
            ..Default::default()
        };

        let result = GithubAppSecretData::from_secret(Arc::new(secret));
        assert!(result.is_ok());

        let auth = result.unwrap();
        assert_eq!(auth.url, "https://github.com/test/repo.git");
        assert_eq!(auth.app_id, 12345);
        assert_eq!(auth.app_installation_id, 67890);
        assert_eq!(
            auth.private_key,
            "-----BEGIN RSA PRIVATE KEY-----\ntest_key\n-----END RSA PRIVATE KEY-----"
        );
    }

    #[test]
    fn test_from_secret_missing_field() {
        let mut string_data = BTreeMap::new();
        string_data.insert(
            "url".to_string(),
            "https://github.com/test/repo.git".to_string(),
        );
        string_data.insert("app_id".to_string(), "12345".to_string());

        let secret = Secret {
            data: None,
            string_data: Some(string_data),
            ..Default::default()
        };

        let result = GithubAppSecretData::from_secret(Arc::new(secret));
        assert!(result.is_err());

        if let Err(SandcastleError::Service { code, message, .. }) = result {
            assert!(matches!(code, ServiceErrorCode::SecretParsingFailed));
            assert!(message.contains("app_installation_id"));
        }
    }

    #[test]
    fn test_from_secret_invalid_app_id() {
        let mut string_data = BTreeMap::new();
        string_data.insert(
            "url".to_string(),
            "https://github.com/test/repo.git".to_string(),
        );
        string_data.insert("app_id".to_string(), "not_a_number".to_string());
        string_data.insert("app_installation_id".to_string(), "67890".to_string());
        string_data.insert("private_key".to_string(), "test_key".to_string());

        let secret = Secret {
            data: None,
            string_data: Some(string_data),
            ..Default::default()
        };

        let result = GithubAppSecretData::from_secret(Arc::new(secret));
        assert!(result.is_err());

        if let Err(SandcastleError::Service { code, message, .. }) = result {
            assert!(matches!(code, ServiceErrorCode::SecretParsingFailed));
            assert!(message.contains("app_id"));
        }
    }
}

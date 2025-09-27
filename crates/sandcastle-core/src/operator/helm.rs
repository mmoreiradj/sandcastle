use k8s_openapi::chrono::{DateTime, Utc};
use semver::Version;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use snafu::{OptionExt, ResultExt};
use std::backtrace::Backtrace;
use std::collections::HashMap;
use std::env::temp_dir;
use std::path::Path;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use validator::{Validate, ValidationError, ValidationErrors};

use crate::error::ServiceErrorCode;
use crate::{
    crd::HelmChartResourceSpec,
    error::{SandcastleError, ValidationSnafu},
};
use sandcastle_utils::validation::validate_k8s_dns_label;

fn validate_api_version_v1(api_version: &str) -> Result<(), validator::ValidationError> {
    if api_version == "v1" {
        Ok(())
    } else {
        Err(validator::ValidationError::new("api_version_must_be_v1"))
    }
}

#[derive(Clone, Debug, Validate)]
pub struct Auth {
    #[validate(length(min = 1))]
    username: String,
    #[validate(length(min = 1))]
    password: String,
}

impl Auth {
    pub fn try_new(username: String, password: String) -> Result<Self, SandcastleError> {
        let auth = Auth { username, password };
        auth.validate().context(ValidationSnafu {
            message: "Invalid auth",
        })?;
        Ok(auth)
    }
}

#[derive(Clone, Debug, Validate)]
pub struct InstallOrUpgradeReleaseRequest {
    #[validate(custom(function = "validate_k8s_dns_label"))]
    release_name: String,
    #[validate(nested)]
    spec: HelmChartResourceSpec,
    #[validate(nested)]
    auth: Option<Auth>,
}

impl InstallOrUpgradeReleaseRequest {
    pub fn try_new(
        release_name: String,
        spec: HelmChartResourceSpec,
        auth: Option<Auth>,
    ) -> Result<Self, SandcastleError> {
        let request = InstallOrUpgradeReleaseRequest {
            release_name,
            spec,
            auth,
        };

        request.validate().context(ValidationSnafu {
            message: "Invalid install or upgrade release request",
        })?;

        Ok(request)
    }
}

#[derive(Clone, Debug, Validate)]
pub struct UninstallReleaseRequest {
    #[validate(custom(function = "validate_k8s_dns_label"))]
    namespace: String,
    #[validate(custom(function = "validate_k8s_dns_label"))]
    release_name: String,
}

impl UninstallReleaseRequest {
    pub fn try_new(namespace: String, release_name: String) -> Result<Self, SandcastleError> {
        let request = UninstallReleaseRequest {
            namespace,
            release_name,
        };
        request.validate().context(ValidationSnafu {
            message: "Invalid uninstall request",
        })?;
        Ok(request)
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct InstallOrUpgradeReleaseResponse {
    name: String,
    last_deployed: DateTime<Utc>,
    namespace: String,
    status: String,
    revision: i64,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct HelmReleaseStatus {
    name: String,
    chart: String,
    version: semver::Version,
    app_version: semver::Version,
    namespace: String,
    revision: i64,
    status: String,
    deployed_at: DateTime<Utc>,
}

impl TryFrom<String> for HelmReleaseStatus {
    type Error = SandcastleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let lines: Vec<&str> = value.lines().collect();
        let mut filtered_lines = Vec::new();
        let mut skip_until_next_field = false;

        for line in lines {
            if line.starts_with("ANNOTATIONS:") || line.starts_with("DEPENDENCIES:") {
                skip_until_next_field = true;
                continue;
            }

            if skip_until_next_field {
                if line.starts_with(char::is_alphabetic)
                    && line.contains(':')
                    && !line.starts_with(' ')
                {
                    skip_until_next_field = false;
                    filtered_lines.push(line);
                }
            } else {
                filtered_lines.push(line);
            }
        }

        let filtered_value = filtered_lines.join("\n");
        println!("filtered_value: {}", filtered_value);

        let helm_release_status: HelmReleaseStatus = serde_yaml::from_str(&filtered_value)
            .whatever_context(format!(
                "Failed to parse helm release status from {}",
                value
            ))?;
        Ok(helm_release_status)
    }
}

#[derive(Clone, Debug, Validate)]
pub struct ReleaseStatusRequest {
    #[validate(custom(function = "validate_k8s_dns_label"))]
    namespace: String,
    #[validate(custom(function = "validate_k8s_dns_label"))]
    release_name: String,
}

impl ReleaseStatusRequest {
    pub fn try_new(namespace: String, release_name: String) -> Result<Self, SandcastleError> {
        let request = ReleaseStatusRequest {
            namespace,
            release_name,
        };
        request.validate().context(ValidationSnafu {
            message: "Invalid release status request",
        })?;
        Ok(request)
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn release_name(&self) -> &str {
        &self.release_name
    }
}

pub trait Helm {
    fn release_status(
        &self,
        request: &ReleaseStatusRequest,
    ) -> impl Future<Output = Result<Option<HelmReleaseStatus>, SandcastleError>> + Send;
    fn install_or_upgrade_release(
        &self,
        request: &InstallOrUpgradeReleaseRequest,
    ) -> impl Future<Output = Result<InstallOrUpgradeReleaseResponse, SandcastleError>> + Send;
    fn uninstall_release(
        &self,
        request: &UninstallReleaseRequest,
    ) -> impl Future<Output = Result<(), SandcastleError>> + Send;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct HelmResult {
    #[serde(rename = "NAME")]
    name: String,
    #[serde(rename = "LAST DEPLOYED")]
    last_deployed: String,
    #[serde(rename = "NAMESPACE")]
    namespace: String,
    #[serde(rename = "STATUS")]
    status: String,
    #[serde(rename = "REVISION")]
    revision: i64,
}

impl TryFrom<String> for HelmResult {
    type Error = SandcastleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let helm_result: HelmResult = serde_yaml::from_str(&value).unwrap();
        Ok(helm_result)
    }
}

impl TryInto<InstallOrUpgradeReleaseResponse> for HelmResult {
    type Error = SandcastleError;

    fn try_into(self) -> Result<InstallOrUpgradeReleaseResponse, Self::Error> {
        let date =
            chrono::NaiveDateTime::parse_from_str(&self.last_deployed, "%a %b %d %H:%M:%S %Y")
                .map_err(|_| {
                    let mut errors = ValidationErrors::new();
                    errors.add(
                        "last_deployed",
                        ValidationError::new("invalid_last_deployed_date"),
                    );
                    SandcastleError::Validation {
                        message: "Invalid last deployed date".to_string(),
                        source: errors,
                        backtrace: Backtrace::capture(),
                    }
                })?;
        Ok(InstallOrUpgradeReleaseResponse {
            name: self.name,
            last_deployed: DateTime::from_naive_utc_and_offset(date, Utc),
            namespace: self.namespace,
            status: self.status,
            revision: self.revision,
        })
    }
}

#[derive(Clone, Debug)]
pub struct HelmCli {
    helm_path: PathBuf,
    http_client: reqwest::Client,
}

impl Default for HelmCli {
    fn default() -> Self {
        Self {
            helm_path: PathBuf::from("helm"),
            http_client: reqwest::Client::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
struct RepositoryIndexResponse {
    #[validate(custom(function = "validate_api_version_v1"))]
    pub api_version: String,
    pub entries: HashMap<String, Vec<RepositoryIndexEntry>>,
}

impl TryFrom<String> for RepositoryIndexResponse {
    type Error = SandcastleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let response: RepositoryIndexResponse = serde_yaml::from_str(&value)
            .whatever_context(format!("Failed to parse repository index from {}", value))?;
        response.validate().context(ValidationSnafu {
            message: "Invalid repository index response",
        })?;
        Ok(response)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
struct RepositoryIndexEntry {
    pub api_version: String,
    #[validate(length(min = 1))]
    pub urls: Vec<String>,
    #[validate(length(min = 1))]
    pub version: String,
    #[validate(length(min = 1))]
    pub digest: String,
}

impl HelmCli {
    pub fn new(helm_path: PathBuf, http_client: reqwest::Client) -> Self {
        Self {
            helm_path,
            http_client,
        }
    }

    fn normalize_repository(repository: &str) -> String {
        repository
            .trim_start_matches("https://")
            .trim_start_matches("http://")
            .trim_end_matches('/')
            .replace(['/', '.', ':'], "-")
            .to_lowercase()
    }

    async fn repository_index(
        &self,
        repository: &str,
    ) -> Result<RepositoryIndexResponse, SandcastleError> {
        let url = format!("{}/index.yaml", repository);
        let response = self
            .http_client
            .get(url.clone())
            .send()
            .await
            .map_err(|e| SandcastleError::Service {
                code: ServiceErrorCode::HelmRepoIndexFailed,
                message: "Failed to get repository index".to_string(),
                reason: format!(
                    "Failed to send request to repository index from {}: {}",
                    url, e
                ),
                backtrace: Backtrace::capture(),
            })?;

        if !response.status().is_success() {
            return Err(SandcastleError::Service {
                code: ServiceErrorCode::HelmRepoIndexFailed,
                message: "Failed to get repository index".to_string(),
                reason: format!(
                    "Unexpected status code from repository index from {}: {}",
                    url,
                    response.status()
                ),
                backtrace: Backtrace::capture(),
            });
        }

        let response = response.text().await.whatever_context(format!(
            "Failed to get response text from repository index from {}",
            url
        ))?;

        let response: RepositoryIndexResponse = response.try_into()?;

        Ok(response)
    }

    /// When the helm cli tries to find the latest version of a chart, it will
    /// find the highest semver version that is not prerelease
    fn find_latest_version(entries: &[RepositoryIndexEntry]) -> Result<&str, SandcastleError> {
        entries
            .iter()
            .filter_map(|entry| {
                Version::parse(&entry.version)
                    .ok()
                    .map(|version| (entry, version))
            })
            .max_by_key(|(_, version)| version.clone())
            .map(|(entry, _)| entry.version.as_str())
            .ok_or(SandcastleError::Service {
                code: ServiceErrorCode::HelmChartVersionNotFound,
                message: "Could not determine latest chart version".to_string(),
                reason: "No chart version found in repository".to_string(),
                backtrace: Backtrace::capture(),
            })
    }

    async fn download_chart(
        &self,
        repository: &str,
        chart: &str,
        version: Option<&str>,
        download_dir: &Path,
    ) -> Result<PathBuf, SandcastleError> {
        let index = self.repository_index(repository).await?;
        let entries = index.entries.get(chart).ok_or(SandcastleError::Service {
            code: ServiceErrorCode::HelmChartNotFound,
            message: "Chart not found in repository".to_string(),
            reason: format!("Chart {} not found in repository {}", chart, repository),
            backtrace: Backtrace::capture(),
        })?;
        let version = match version {
            Some(v) => v,
            None => Self::find_latest_version(entries)?,
        };
        let entry = index
            .entries
            .get(chart)
            .ok_or(SandcastleError::Service {
                code: ServiceErrorCode::HelmChartNotFound,
                message: "Chart not found in repository".to_string(),
                reason: format!("Chart {} not found in repository {}", chart, repository),
                backtrace: Backtrace::capture(),
            })?
            .iter()
            // will fetch latest version if version is None
            .find(|entry| version == "latest" || entry.version == version)
            .ok_or(SandcastleError::Service {
                code: ServiceErrorCode::HelmChartVersionNotFound,
                message: "Chart version not found in repository".to_string(),
                reason: format!(
                    "Chart {} version {} not found in repository {}",
                    chart, version, repository
                ),
                backtrace: Backtrace::capture(),
            })?;

        let url = entry.urls.first().whatever_context(format!(
            "Version {} not found in repository {}",
            version, repository
        ))?;

        let response = self
            .http_client
            .get(url.clone())
            .send()
            .await
            .whatever_context(format!("Failed to download chart from {}", url))?;

        let file_path = download_dir.join(format!(
            "{}-{}-{}.tgz",
            uuid::Uuid::new_v4(),
            chart,
            version
        ));
        let mut file = File::create(file_path.clone())
            .await
            .whatever_context(format!("Failed to create file for chart {}", chart))?;

        let bytes = response
            .bytes()
            .await
            .whatever_context(format!("Failed to read response bytes from {}", url))?;

        file.write_all(&bytes).await.whatever_context(format!(
            "Failed to write response bytes to file for chart {}",
            chart
        ))?;

        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        let actual_hash = format!("{:x}", hasher.finalize());

        if actual_hash != entry.digest {
            return Err(SandcastleError::Service {
                code: ServiceErrorCode::HelmChartDownloadFailed,
                message: "Chart integrity check failed".to_string(),
                reason: format!(
                    "SHA256 hash mismatch for chart {} version {}. Expected: {}, Got: {}",
                    chart, version, entry.digest, actual_hash
                ),
                backtrace: Backtrace::capture(),
            });
        }

        Ok(file_path)
    }

    async fn cleanup_download_chart(file_path: &Path) -> Result<(), SandcastleError> {
        std::fs::remove_file(file_path).whatever_context(format!(
            "Failed to remove file for chart {}",
            file_path.display()
        ))?;
        Ok(())
    }
}

impl Helm for HelmCli {
    #[tracing::instrument(skip(self))]
    async fn release_status(
        &self,
        request: &ReleaseStatusRequest,
    ) -> Result<Option<HelmReleaseStatus>, SandcastleError> {
        tracing::debug!("getting helm chart status");
        let output = Command::new(&self.helm_path)
            .arg("get")
            .arg("metadata")
            .arg(request.release_name.clone())
            .arg("--namespace")
            .arg(request.namespace.clone())
            .output()
            .await
            .whatever_context(
                "Failed to start the command to run get helm chart status".to_string(),
            )?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr).to_string();
            if error.contains("Error: release: not found") {
                return Ok(None);
            }
            return Err(SandcastleError::Service {
                code: ServiceErrorCode::HelmReleaseStatusFailed,
                message: "Failed to get helm chart status".to_string(),
                reason: format!("Failed to get helm chart status: {}", error),
                backtrace: Backtrace::capture(),
            });
        }

        let stdout_str = String::from_utf8_lossy(&output.stdout).to_string();
        let result = HelmReleaseStatus::try_from(stdout_str)?;

        Ok(Some(result))
    }

    #[tracing::instrument(skip(self))]
    async fn install_or_upgrade_release(
        &self,
        request: &InstallOrUpgradeReleaseRequest,
    ) -> Result<InstallOrUpgradeReleaseResponse, SandcastleError> {
        tracing::debug!("installing or upgrading helm chart");
        let chart: String = if let Some(repository) = &request.spec.repository {
            self.download_chart(
                repository,
                &request.spec.chart,
                request.spec.version.as_deref(),
                &temp_dir(),
            )
            .await?
            .display()
            .to_string()
        } else {
            request.spec.chart.clone()
        };

        let mut command = Command::new(&self.helm_path);
        command
            .arg("upgrade")
            .arg("--install")
            .arg("--create-namespace")
            .arg("--namespace")
            .arg(&request.spec.namespace)
            .arg(&request.release_name)
            .arg(&chart);

        if let Some(sets) = &request.spec.set {
            for set in sets {
                command
                    .arg("--set")
                    .arg(format!("{}={}", set.path, set.value));
            }
        }

        let output = command.output().await.whatever_context(
            "Failed to start the command to run install or upgrade helm chart".to_string(),
        )?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(SandcastleError::Service {
                code: ServiceErrorCode::HelmInstallOrUpgradeFailed,
                message: "Failed to install or upgrade helm chart".to_string(),
                reason: format!("Failed to install or upgrade helm chart: {}", error),
                backtrace: Backtrace::capture(),
            });
        }

        let stdout_str = String::from_utf8_lossy(&output.stdout).to_string();
        let first_part = stdout_str
            .split("---")
            .next()
            .whatever_context("Failed to parse helm result from stdout".to_string())?;

        let helm_result: HelmResult = serde_yaml::from_str(first_part).whatever_context(
            "Failed to parse helm result from stdout of install or upgrade helm chart".to_string(),
        )?;
        let response: InstallOrUpgradeReleaseResponse = helm_result.try_into()?;

        if let Err(e) = Self::cleanup_download_chart(Path::new(&chart)).await {
            tracing::error!("Failed to cleanup downloaded chart: {}", e);
        };

        Ok(response)
    }

    #[tracing::instrument(skip(self))]
    async fn uninstall_release(
        &self,
        request: &UninstallReleaseRequest,
    ) -> Result<(), SandcastleError> {
        tracing::debug!("uninstalling helm chart");

        let mut command = Command::new(&self.helm_path);
        command
            .arg("uninstall")
            .arg(request.release_name.clone())
            .arg("--namespace")
            .arg(request.namespace.clone());

        let output = command.output().await.whatever_context(
            "Failed to start the command to run uninstall helm chart".to_string(),
        )?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(SandcastleError::Service {
                code: ServiceErrorCode::HelmUninstallFailed,
                message: "Failed to uninstall helm chart".to_string(),
                reason: format!("Failed to uninstall helm chart: {}", error),
                backtrace: Backtrace::capture(),
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use mockito::{Mock, ServerGuard};

    use super::*;
    use googletest::prelude::*;

    fn valid_yaml_index(base_url: &str) -> String {
        format!(
            r#"
apiVersion: v1
entries:
  minio-operator:
    - apiVersion: v2
      appVersion: v4.3.7
      created: "2025-05-05T19:21:40.589983611-07:00"
      description: A Helm chart for MinIO Operator
      digest: 594f746a54d6ced86b0147135afed425c453e015a15228b634bd79add0d24982
      home: {base_url}
      icon: {base_url}/resources/img/logo/MINIO_wordmark.png
      keywords:
        - storage
        - object-storage
        - S3
      maintainers:
        - email: dev@minio.io
          name: MinIO, Inc
      name: minio-operator
      sources:
        - https://github.com/minio/operator
      type: application
      urls:
        - {base_url}/helm-releases/minio-operator-4.3.7.tgz
      version: 4.3.7
    - apiVersion: v2
      appVersion: v4.3.6
      created: "2025-05-05T19:21:40.587677327-07:00"
      description: A Helm chart for MinIO Operator
      digest: 15bb40e086f5e562b7c588dac48a5399fadc1b9f6895f913bbd5a2993c683da7
      home: {base_url}
      icon: {base_url}/resources/img/logo/MINIO_wordmark.png
      keywords:
        - storage
        - object-storage
        - S3
      maintainers:
        - email: dev@minio.io
          name: MinIO, Inc
      name: minio-operator
      sources:
        - https://github.com/minio/operator
      type: application
      urls:
        - {base_url}/helm-releases/minio-operator-4.3.6.tgz
      version: 4.3.6
generated: "2025-05-05T19:21:40.518077312-07:00"
"#
        )
    }

    async fn mock_repo_server() -> (ServerGuard, Mock, Mock) {
        let mut server = mockito::Server::new_async().await;
        let index_yaml = include_str!("../../tests/fixtures/index.yaml");
        let index_yaml = index_yaml.replace("BASE_URL", &server.url());
        let index_mock = server
            .mock("GET", "/index.yaml")
            .with_body(index_yaml)
            .create_async()
            .await;
        let chart_mock = server
            .mock("GET", "/helm-releases/classic-chart-0.1.0.tgz")
            .with_body(include_bytes!(
                "../../tests/fixtures/classic-chart-0.1.0.tgz"
            ))
            .create_async()
            .await;
        (server, index_mock, chart_mock)
    }

    #[test_log::test(tokio::test)]
    pub async fn test_normalize_repository() {
        let cases = vec![
            (
                "https://kubernetes.github.io/ingress-nginx",
                "kubernetes-github-io-ingress-nginx",
            ),
            (
                "http://kubernetes.github.io/ingress-nginx",
                "kubernetes-github-io-ingress-nginx",
            ),
            (
                "https://kubernetes.github.io/ingress-nginx/",
                "kubernetes-github-io-ingress-nginx",
            ),
            (
                "https://kubernetes.github.io/ingress-nginx//",
                "kubernetes-github-io-ingress-nginx",
            ),
            (
                "https://kubernetes.github.io/ingress-nginx//",
                "kubernetes-github-io-ingress-nginx",
            ),
        ];

        for (repository, expected) in cases {
            let normalized = HelmCli::normalize_repository(repository);
            assert_eq!(
                normalized, expected,
                "Failed to normalize repository. Got: {}, expected: {}",
                normalized, expected
            );
        }
    }

    #[gtest]
    #[test_log::test(tokio::test)]
    pub async fn test_index_serialization_success() -> Result<()> {
        let index_yaml = valid_yaml_index("https://operator.min.io");

        let index_response: RepositoryIndexResponse = index_yaml.try_into()?;
        verify_that!(index_response.api_version, eq("v1"))?;
        let entries = index_response.entries.get("minio-operator").unwrap();
        verify_that!(
            entries,
            elements_are![
                matches_pattern!(RepositoryIndexEntry {
                    api_version: eq("v2"),
                    urls: elements_are![eq(
                        "https://operator.min.io/helm-releases/minio-operator-4.3.7.tgz"
                    )],
                    version: eq("4.3.7"),
                    digest: eq("594f746a54d6ced86b0147135afed425c453e015a15228b634bd79add0d24982"),
                }),
                matches_pattern!(RepositoryIndexEntry {
                    api_version: eq("v2"),
                    urls: elements_are![eq(
                        "https://operator.min.io/helm-releases/minio-operator-4.3.6.tgz"
                    )],
                    version: eq("4.3.6"),
                    digest: eq("15bb40e086f5e562b7c588dac48a5399fadc1b9f6895f913bbd5a2993c683da7"),
                })
            ]
        )
    }

    #[gtest]
    #[test_log::test(tokio::test)]
    pub async fn test_index_serialization_failure_unsupported_api_version() -> Result<()> {
        let index_yaml = r#"apiVersion: v2
entries:
  minio-operator:
    - apiVersion: v2
      appVersion: v4.3.7
      created: "2025-05-05T19:21:40.589983611-07:00"
      description: A Helm chart for MinIO Operator
      digest: 594f746a54d6ced86b0147135afed425c453e015a15228b634bd79add0d24982
      home: https://min.io
      icon: https://min.io/resources/img/logo/MINIO_wordmark.png
      keywords:
        - storage
        - object-storage
        - S3
      maintainers:
        - email: dev@minio.io
          name: MinIO, Inc
      name: minio-operator
      sources:
        - https://github.com/minio/operator
      type: application
      urls:
        - https://operator.min.io/helm-releases/minio-operator-4.3.7.tgz
      version: 4.3.7
"#;
        let index_response = RepositoryIndexResponse::try_from(index_yaml.to_string());
        verify_that!(
            index_response,
            err(displays_as(contains_substring("api_version_must_be_v1")))
        )
    }

    #[gtest]
    #[test_log::test(tokio::test)]
    pub async fn test_repository_index() -> Result<()> {
        let (server, index_mock, _) = mock_repo_server().await;

        let url = server.url();
        let helm_cli = HelmCli::default();
        let index_response = helm_cli.repository_index(&url).await;
        index_mock.assert_async().await;
        verify_that!(index_response, ok(anything()))?;
        let index_response = index_response.unwrap();
        verify_that!(index_response.api_version, eq("v1"))?;
        let entries = index_response
            .entries
            .get("classic-chart")
            .expect("classic-chart not found");
        verify_that!(
            entries,
            elements_are![matches_pattern!(RepositoryIndexEntry {
                api_version: eq("v2"),
                urls: elements_are![eq(&format!("{url}/helm-releases/classic-chart-0.1.0.tgz"))],
                version: eq("0.1.0"),
                digest: eq("22fc47d859a86b61e8803cddcc68f7dc8aea878030095dd0c3643b0373ace0c3"),
            }),]
        )
    }

    #[gtest]
    #[test_log::test(tokio::test)]
    pub async fn test_helm_install_or_upgrade_response_parsing_success() -> Result<()> {
        let response = r#"NAME: mychart
LAST DEPLOYED: Fri Sep 19 16:52:06 2025
NAMESPACE: default
STATUS: pending-install
REVISION: 1
HOOKS:
"#;
        let helm_result = HelmResult::try_from(response.to_string());
        verify_that!(
            helm_result,
            ok(matches_pattern!(HelmResult {
                name: eq("mychart"),
                last_deployed: eq("Fri Sep 19 16:52:06 2025"),
                namespace: eq("default"),
                status: eq("pending-install"),
                revision: eq(&1i64),
            }))
        )
    }

    #[gtest]
    #[test_log::test(tokio::test)]
    pub async fn test_download_chart_latest_version_success() -> Result<()> {
        let (server, index_mock, chart_mock) = mock_repo_server().await;
        let url = server.url();
        let helm_cli = HelmCli::default();

        let response = helm_cli
            .download_chart(&url, "classic-chart", None, &temp_dir())
            .await;
        index_mock.assert_async().await;
        chart_mock.assert_async().await;
        verify_that!(
            response,
            ok(predicate(|path: &PathBuf| path
                .to_string_lossy()
                .contains("classic-chart-0.1.0.tgz")))
        )?;
        let response = response.unwrap();
        verify_that!(Path::new(&response).exists(), eq(true))?;
        HelmCli::cleanup_download_chart(&Path::new(&response))
            .await
            .expect("Failed to cleanup downloaded chart");
        Ok(())
    }

    #[gtest]
    #[test_log::test(tokio::test)]
    pub async fn test_download_chart_specific_version_success() -> Result<()> {
        let (server, index_mock, chart_mock) = mock_repo_server().await;
        let url = server.url();
        let helm_cli = HelmCli::default();

        let response = helm_cli
            .download_chart(&url, "classic-chart", Some("0.1.0"), &temp_dir())
            .await;
        index_mock.assert_async().await;
        chart_mock.assert_async().await;
        verify_that!(
            response,
            ok(predicate(|path: &PathBuf| path
                .to_string_lossy()
                .contains("classic-chart-0.1.0.tgz")))
        )?;
        let response = response.unwrap();
        verify_that!(Path::new(&response).exists(), eq(true))?;
        HelmCli::cleanup_download_chart(&Path::new(&response))
            .await
            .expect("Failed to cleanup downloaded chart");
        Ok(())
    }

    #[gtest]
    #[test_log::test(tokio::test)]
    pub async fn test_helm_release_status() -> Result<()> {
        let response = r#"NAME: keycloak
CHART: keycloak
VERSION: 21.7.1
APP_VERSION: 24.0.5
ANNOTATIONS: category=DeveloperTools,images=- name: keycloak
  image: docker.io/bitnami/keycloak:24.0.5-debian-12-r3
- name: keycloak-config-cli
  image: docker.io/bitnami/keycloak-config-cli:5.12.0-debian-12-r6
,licenses=Apache-2.0
DEPENDENCIES: common
NAMESPACE: keycloak
REVISION: 9
STATUS: deployed
DEPLOYED_AT: 2024-07-27T01:23:35+03:00"#;
        let helm_release_status = HelmReleaseStatus::try_from(response.to_string());
        verify_that!(
            helm_release_status,
            ok(matches_pattern!(HelmReleaseStatus {
                name: eq("keycloak"),
                chart: eq("keycloak"),
                version: eq(&semver::Version::parse("21.7.1").unwrap()),
                app_version: eq(&semver::Version::parse("24.0.5").unwrap()),
                namespace: eq("keycloak"),
                revision: eq(&9i64),
                status: eq("deployed"),
                deployed_at: eq(&chrono::DateTime::parse_from_str(
                    "2024-07-27T01:23:35+03:00",
                    "%Y-%m-%dT%H:%M:%S%z"
                )
                .unwrap()
                .with_timezone(&Utc)),
            }))
        )
    }
}

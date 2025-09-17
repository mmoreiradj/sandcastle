use std::collections::HashMap;

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[kube(
    group = "sandcastle.dev",
    version = "v1",
    kind = "SandcastleProject",
    namespaced
)]
pub struct SandcastleProjectSpec {
    /// The resources to deploy in the project
    pub resources: Vec<SandcastleProjectResource>,
    /// The destination to deploy the project to
    pub destination: SandcastleProjectDestination,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct SandcastleProjectResource {
    /// The name of the resource, usage will depend on the kind
    pub name: String,
    /// The kind of the resource
    #[serde(flatten)]
    pub kind: SandcastleProjectResourceKind,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(tag = "kind", content = "spec")]
pub enum SandcastleProjectResourceKind {
    HelmChart(HelmChartResourceSpec),
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
/// Represents a Helm chart resource
pub struct HelmChartResourceSpec {
    /// The chart to deploy
    pub chart: String,
    /// The repository to deploy the chart from
    /// If chart is prefixed by file://, it is assumed to be the url to a git repository
    /// If not provided, chart is assumed to be in an OCI repository
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    /// The version of the chart to deploy
    pub version: String,
    /// The overrides to pass to the chart, takes precedence over values
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct SandcastleProjectDestination {
    /// The namespace to deploy the project to
    pub namespace: String,
    /// The cluster to deploy the project to
    pub server: String,
}

#[cfg(test)]
mod tests {
    use kube::api::ObjectMeta;

    use super::*;

    #[test]
    fn test_crd_yaml() {
        let spec = SandcastleProjectSpec {
            destination: SandcastleProjectDestination {
                namespace: "default".to_string(),
                server: "https://kubernetes.example.com".to_string(),
            },
            resources: vec![
                SandcastleProjectResource {
                    name: "web-server".to_string(),
                    kind: SandcastleProjectResourceKind::HelmChart(HelmChartResourceSpec {
                        chart: "ingress-nginx".to_string(),
                        repository: Some("https://kubernetes.github.io/ingress-nginx".to_string()),
                        version: "4.8.0".to_string(),
                        overrides: None,
                    }),
                },
                SandcastleProjectResource {
                    name: "database".to_string(),
                    kind: SandcastleProjectResourceKind::HelmChart(HelmChartResourceSpec {
                        chart: "ingress-nginx".to_string(),
                        repository: Some("https://kubernetes.github.io/ingress-nginx".to_string()),
                        version: "4.8.0".to_string(),
                        overrides: None,
                    }),
                },
                SandcastleProjectResource {
                    name: "app".to_string(),
                    kind: SandcastleProjectResourceKind::HelmChart(HelmChartResourceSpec {
                        chart: "oci://my-registry.com/my-charts/my-app".to_string(),
                        repository: None,
                        version: "4.8.0".to_string(),
                        overrides: Some([("key".to_string(), "value".to_string())].into()),
                    }),
                },
            ],
        };

        let crd = SandcastleProject {
            metadata: ObjectMeta {
                name: Some("test".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            },
            spec,
        };

        let yaml = serde_yaml::to_string(&crd).unwrap();
        insta::assert_snapshot!(yaml);
    }
}

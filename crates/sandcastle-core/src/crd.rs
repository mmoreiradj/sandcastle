use kube::CustomResource;
use sandcastle_utils::serde::option_bool_true;
use sandcastle_utils::validation::{validate_k8s_dns_label, validate_k8s_dns_subdomain};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(CustomResource, Debug, Serialize, Deserialize, Clone, JsonSchema, Validate)]
#[kube(
    group = "sandcastle.dev",
    version = "v1",
    kind = "SandcastleProject",
    namespaced
)]
pub struct SandcastleProjectSpec {
    /// The resources to deploy in the project
    #[validate(nested)]
    pub resources: Vec<SandcastleProjectResource>,
    /// The destination to deploy the project to
    #[validate(nested)]
    pub destination: SandcastleProjectDestination,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Validate)]
pub struct SandcastleProjectResource {
    /// The name of the resource, usage will depend on the kind
    #[validate(custom(function = "validate_k8s_dns_label"))]
    pub name: String,
    /// The kind of the resource
    #[serde(flatten)]
    #[validate(nested)]
    pub kind: SandcastleProjectResourceKind,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(tag = "kind", content = "spec")]
pub enum SandcastleProjectResourceKind {
    HelmChart(HelmChartResourceSpec),
}

impl Validate for SandcastleProjectResourceKind {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            SandcastleProjectResourceKind::HelmChart(spec) => spec.validate(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Validate)]
#[validate(schema(
    function = "validate_helm_chart_resource_spec",
    skip_on_field_errors = true
))]
/// Represents a Helm chart resource
pub struct HelmChartResourceSpec {
    /// The chart to deploy
    #[validate(length(min = 1))]
    pub chart: String,
    /// The repository to deploy the chart from
    /// If chart is prefixed by file://, it is assumed to be the url to a git repository
    /// If not provided, chart is assumed to be in an OCI repository
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    /// The version of the chart to deploy
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    pub version: Option<String>,
    /// The overrides to pass to the chart, takes precedence over values
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<Vec<HelmChartResourceSet>>,
    /// The namespace to deploy the chart to
    #[validate(custom(function = "validate_k8s_dns_subdomain"))]
    pub namespace: String,
    /// Wether to create a namespace if it doesn't exist
    #[serde(default = "option_bool_true", skip_serializing_if = "Option::is_none")]
    pub create_namespace: Option<bool>,
}

fn validate_helm_chart_resource_spec(spec: &HelmChartResourceSpec) -> Result<(), ValidationError> {
    spec.validate_oci_requirement()
}

impl HelmChartResourceSpec {
    fn validate_oci_requirement(&self) -> Result<(), ValidationError> {
        if self.repository.is_none() && !self.chart.starts_with("oci://") {
            return Err(ValidationError::new("missing_oci_prefix").with_message(
                "Chart is expected to be prefixed with oci:// when repository is not provided"
                    .into(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Validate)]
pub struct HelmChartResourceSet {
    /// The path to the value to override
    #[validate(length(min = 1))]
    pub path: String,
    /// The value of the override
    #[validate(length(min = 1))]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Validate)]
pub struct SandcastleProjectDestination {
    /// The namespace to deploy the project to
    #[validate(custom(function = "validate_k8s_dns_subdomain"))]
    pub namespace: String,
    /// The cluster to deploy the project to
    #[validate(url)]
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
                        namespace: "default".to_string(),
                        chart: "ingress-nginx".to_string(),
                        repository: Some("https://kubernetes.github.io/ingress-nginx".to_string()),
                        version: Some("4.8.0".to_string()),
                        set: None,
                        create_namespace: None,
                    }),
                },
                SandcastleProjectResource {
                    name: "database".to_string(),
                    kind: SandcastleProjectResourceKind::HelmChart(HelmChartResourceSpec {
                        namespace: "default".to_string(),
                        chart: "ingress-nginx".to_string(),
                        repository: Some("https://kubernetes.github.io/ingress-nginx".to_string()),
                        version: Some("4.8.0".to_string()),
                        set: Some(vec![HelmChartResourceSet {
                            path: "key".to_string(),
                            value: "value".to_string(),
                        }]),
                        create_namespace: None,
                    }),
                },
                SandcastleProjectResource {
                    name: "app".to_string(),
                    kind: SandcastleProjectResourceKind::HelmChart(HelmChartResourceSpec {
                        namespace: "default".to_string(),
                        chart: "oci://my-registry.com/my-charts/my-app".to_string(),
                        repository: None,
                        version: None,
                        set: Some(vec![HelmChartResourceSet {
                            path: "key".to_string(),
                            value: "value".to_string(),
                        }]),
                        create_namespace: Some(true),
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

    #[test]
    fn test_successful_helm_chart_resource_spec() {
        let cases = vec![
            (
                "default".to_string(),
                "nginx".to_string(),
                Some("https://kubernetes.github.io/ingress-nginx".to_string()),
                Some("1.0.0".to_string()),
                None,
            ),
            (
                "kube-system".to_string(),
                "oci://my-registry.com/stable/redis".to_string(),
                None,
                Some("v2.1.0".to_string()),
                Some(vec![HelmChartResourceSet {
                    path: "override".to_string(),
                    value: "true".to_string(),
                }]),
            ),
        ];

        for (namespace, chart, repository, version, set) in cases {
            let helm_spec = HelmChartResourceSpec {
                namespace,
                chart,
                repository,
                version,
                set,
                create_namespace: None,
            };
            assert!(
                helm_spec.validate().is_ok(),
                "Failed to validate helm chart spec. Got err: {:?}",
                helm_spec.validate().err()
            );
        }
    }

    #[test]
    fn test_invalid_helm_chart_spec() {
        let cases = vec![
            // Invalid namespace
            HelmChartResourceSpec {
                namespace: "invalid-*-namespace".to_string(),
                chart: "nginx".to_string(),
                repository: Some("https://kubernetes.github.io/ingress-nginx".to_string()),
                version: Some("1.0.0".to_string()),
                set: None,
                create_namespace: None,
            },
            // Empty namespace
            HelmChartResourceSpec {
                namespace: "".to_string(),
                chart: "nginx".to_string(),
                repository: Some("https://kubernetes.github.io/ingress-nginx".to_string()),
                version: Some("1.0.0".to_string()),
                set: None,
                create_namespace: None,
            },
            // Empty chart
            HelmChartResourceSpec {
                namespace: "default".to_string(),
                chart: "".to_string(),
                repository: Some("https://kubernetes.github.io/ingress-nginx".to_string()),
                version: None,
                set: None,
                create_namespace: None,
            },
            // Empty version when provided
            HelmChartResourceSpec {
                namespace: "default".to_string(),
                chart: "nginx".to_string(),
                repository: Some("https://kubernetes.github.io/ingress-nginx".to_string()),
                version: Some("".to_string()),
                set: None,
                create_namespace: None,
            },
        ];

        for helm_spec in cases {
            assert!(
                helm_spec.validate().is_err(),
                "Expected error for helm chart spec. Got ok for: {:?}",
                helm_spec
            );
        }
    }

    #[test]
    fn test_missing_oci_prefix() {
        let helm_spec = HelmChartResourceSpec {
            namespace: "default".to_string(),
            chart: "nginx".to_string(),
            repository: None,
            version: Some("1.0.0".to_string()),
            set: None,
            create_namespace: None,
        };

        let result = helm_spec.validate();
        assert!(
            result.is_err(),
            "Expected error for helm chart spec without repository or oci:// prefix. Got ok"
        );

        let err = result.err().unwrap();
        let errors = err.field_errors();
        assert!(
            errors.contains_key("__all__"),
            "Should contain validation error for missing OCI prefix"
        );
    }

    #[test]
    fn test_successful_project_destination() {
        let cases = vec![
            ("default".to_string(), "https://example.com".to_string()),
            ("1234567890".to_string(), "https://example.com".to_string()),
            (
                "my-namespace-2".to_string(),
                "http://example.com".to_string(),
            ),
        ];
        for (namespace, server) in cases {
            let destination = SandcastleProjectDestination { namespace, server };
            assert!(
                destination.validate().is_ok(),
                "Failed to validate project destination. Got err: {:?}",
                destination.validate().err()
            );
        }
    }

    #[test]
    fn test_invalid_project_destination() {
        let cases = vec![
            // Invalid URL
            SandcastleProjectDestination {
                namespace: "default".to_string(),
                server: "example.com".to_string(),
            },
            // Invalid namespace
            SandcastleProjectDestination {
                namespace: "-namespace".to_string(),
                server: "http://example.com".to_string(),
            },
            // Invalid namespace with special characters
            SandcastleProjectDestination {
                namespace: "my-*-namespace".to_string(),
                server: "http://example.com".to_string(),
            },
        ];
        for destination in cases {
            assert!(
                destination.validate().is_err(),
                "Expected error for project destination. Got ok: {:?}",
                destination
            );
        }
    }

    #[test]
    fn test_successful_project_resource() {
        let helm_spec = HelmChartResourceSpec {
            namespace: "default".to_string(),
            chart: "nginx".to_string(),
            repository: Some("https://kubernetes.github.io/ingress-nginx".to_string()),
            version: Some("1.0.0".to_string()),
            set: None,
            create_namespace: None,
        };

        let cases = vec![
            (
                "web-server".to_string(),
                SandcastleProjectResourceKind::HelmChart(helm_spec.clone()),
            ),
            (
                "database".to_string(),
                SandcastleProjectResourceKind::HelmChart(helm_spec.clone()),
            ),
        ];

        for (name, kind) in cases {
            let resource = SandcastleProjectResource { name, kind };
            assert!(
                resource.validate().is_ok(),
                "Failed to validate project resource. Got err: {:?}",
                resource.validate().err()
            );
        }
    }

    #[test]
    fn test_invalid_project_resource() {
        let helm_spec = HelmChartResourceSpec {
            namespace: "default".to_string(),
            chart: "nginx".to_string(),
            repository: Some("https://kubernetes.github.io/ingress-nginx".to_string()),
            version: Some("1.0.0".to_string()),
            set: None,
            create_namespace: None,
        };

        let cases = vec![
            // Invalid resource names
            SandcastleProjectResource {
                name: "invalid-*-name".to_string(),
                kind: SandcastleProjectResourceKind::HelmChart(helm_spec.clone()),
            },
            SandcastleProjectResource {
                name: "-invalid".to_string(),
                kind: SandcastleProjectResourceKind::HelmChart(helm_spec.clone()),
            },
            SandcastleProjectResource {
                name: "invalid-".to_string(),
                kind: SandcastleProjectResourceKind::HelmChart(helm_spec.clone()),
            },
        ];

        for resource in cases {
            assert!(
                resource.validate().is_err(),
                "Expected error for project resource. Got ok: {:?}",
                resource
            );
        }
    }

    #[test]
    fn test_successful_sandcastle_project() {
        let helm_spec = HelmChartResourceSpec {
            namespace: "default".to_string(),
            chart: "nginx".to_string(),
            repository: Some("https://kubernetes.github.io/ingress-nginx".to_string()),
            version: Some("1.0.0".to_string()),
            set: None,
            create_namespace: None,
        };

        let resource = SandcastleProjectResource {
            name: "web-server".to_string(),
            kind: SandcastleProjectResourceKind::HelmChart(helm_spec),
        };

        let destination = SandcastleProjectDestination {
            namespace: "default".to_string(),
            server: "https://kubernetes.example.com".to_string(),
        };

        let spec = SandcastleProjectSpec {
            resources: vec![resource],
            destination,
        };

        assert!(
            spec.validate().is_ok(),
            "Failed to validate sandcastle project spec. Got err: {:?}",
            spec.validate().err()
        );
    }

    #[test]
    fn test_helm_chart_resource_set_validation() {
        // Valid set
        let valid_set = HelmChartResourceSet {
            path: "key".to_string(),
            value: "value".to_string(),
        };
        assert!(valid_set.validate().is_ok());

        // Invalid sets
        let invalid_sets = vec![
            HelmChartResourceSet {
                path: "".to_string(),
                value: "value".to_string(),
            },
            HelmChartResourceSet {
                path: "key".to_string(),
                value: "".to_string(),
            },
        ];

        for set in invalid_sets {
            assert!(
                set.validate().is_err(),
                "Expected error for helm chart resource set: {:?}",
                set
            );
        }
    }

    #[test]
    fn test_oci_chart_without_repository() {
        let helm_spec = HelmChartResourceSpec {
            namespace: "default".to_string(),
            chart: "oci://my-registry.com/stable/redis".to_string(),
            repository: None,
            version: Some("v2.1.0".to_string()),
            set: Some(vec![HelmChartResourceSet {
                path: "override".to_string(),
                value: "true".to_string(),
            }]),
            create_namespace: Some(true),
        };

        assert!(
            helm_spec.validate().is_ok(),
            "OCI chart should be valid without repository. Got err: {:?}",
            helm_spec.validate().err()
        );
    }
}

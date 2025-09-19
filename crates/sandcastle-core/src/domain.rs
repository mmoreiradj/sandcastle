use std::backtrace::Backtrace;

use sandcastle_utils::validation::{validate_k8s_dns_label, validate_k8s_dns_subdomain};
use snafu::ResultExt;
use validator::{Validate, ValidationError, ValidationErrors};

use crate::{
    crd::SandcastleProjectSpec,
    error::{SandcastleProjectError, ValidationSnafu},
};

#[derive(Clone, Debug, Validate)]
pub struct SandcastleProject {
    #[validate(nested)]
    resources: Vec<SandcastleProjectResource>,
    #[validate(nested)]
    destination: SandcastleProjectDestination,
}

impl SandcastleProject {
    pub fn try_new(
        resources: Vec<SandcastleProjectResource>,
        destination: SandcastleProjectDestination,
    ) -> Result<Self, SandcastleProjectError> {
        let project = SandcastleProject {
            resources,
            destination,
        };
        project.validate().context(ValidationSnafu {
            message: "Invalid sandcastle project",
        })?;
        Ok(project)
    }

    pub fn resources(&self) -> &[SandcastleProjectResource] {
        &self.resources
    }

    pub fn destination(&self) -> &SandcastleProjectDestination {
        &self.destination
    }
}

impl TryFrom<SandcastleProjectSpec> for SandcastleProject {
    type Error = SandcastleProjectError;

    fn try_from(spec: SandcastleProjectSpec) -> Result<Self, Self::Error> {
        let resources = spec
            .resources
            .into_iter()
            .map(|resource| {
                let kind = match resource.kind {
                    crate::crd::SandcastleProjectResourceKind::HelmChart(helm_spec) => {
                        let helm_chart = HelmChartResourceSpec::try_new(
                            spec.destination.namespace.clone(),
                            resource.name.clone(),
                            helm_spec.chart,
                            helm_spec.repository,
                            helm_spec.version,
                            helm_spec.set,
                        )?;
                        SandcastleProjectResourceKind::HelmChart(helm_chart)
                    }
                };
                SandcastleProjectResource::try_new(resource.name, kind)
            })
            .collect::<Result<Vec<_>, _>>()?;

        let destination = SandcastleProjectDestination::try_new(
            spec.destination.namespace,
            spec.destination.server,
        )?;

        SandcastleProject::try_new(resources, destination)
    }
}

#[derive(Clone, Debug, Validate)]
pub struct SandcastleProjectResource {
    #[validate(custom(function = "validate_k8s_dns_label"))]
    name: String,
    #[validate(nested)]
    kind: SandcastleProjectResourceKind,
}

impl SandcastleProjectResource {
    pub fn try_new(
        name: String,
        kind: SandcastleProjectResourceKind,
    ) -> Result<Self, SandcastleProjectError> {
        let resource = SandcastleProjectResource { name, kind };
        resource.validate().context(ValidationSnafu {
            message: "Invalid sandcastle project resource",
        })?;
        Ok(resource)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> &SandcastleProjectResourceKind {
        &self.kind
    }
}

#[derive(Clone, Debug)]
pub enum SandcastleProjectResourceKind {
    HelmChart(HelmChartResourceSpec),
}

impl Validate for SandcastleProjectResourceKind {
    fn validate(&self) -> Result<(), ValidationErrors> {
        match self {
            SandcastleProjectResourceKind::HelmChart(spec) => spec.validate(),
        }
    }
}

#[derive(Clone, Debug, Validate)]
pub struct HelmChartResourceSpec {
    #[validate(custom(function = "validate_k8s_dns_subdomain"))]
    namespace: String,
    #[validate(custom(function = "validate_k8s_dns_label"))]
    release_name: String,
    #[validate(length(min = 1))]
    chart: String,
    repository: Option<String>,
    #[validate(length(min = 1))]
    version: Option<String>,
    #[validate(nested)]
    set: Option<Vec<HelmChartResourceSet>>,
}

#[derive(Clone, Debug, Validate)]
pub struct HelmChartResourceSet {
    #[validate(length(min = 1))]
    path: String,
    #[validate(length(min = 1))]
    value: String,
}

impl HelmChartResourceSet {
    pub fn try_new(path: String, value: String) -> Result<Self, SandcastleProjectError> {
        let set = HelmChartResourceSet { path, value };
        set.validate().context(ValidationSnafu {
            message: "Invalid helm chart resource set",
        })?;
        Ok(set)
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl HelmChartResourceSpec {
    pub fn try_new(
        namespace: String,
        release_name: String,
        chart: String,
        repository: Option<String>,
        version: Option<String>,
        set: Option<Vec<crate::crd::HelmChartResourceSet>>,
    ) -> Result<Self, SandcastleProjectError> {
        let set = if let Some(crd_set) = set {
            let domain_set = crd_set
                .into_iter()
                .map(|s| HelmChartResourceSet::try_new(s.path, s.value))
                .collect::<Result<Vec<_>, _>>()?;
            Some(domain_set)
        } else {
            None
        };

        let helm_chart = HelmChartResourceSpec {
            namespace,
            release_name,
            chart: chart.clone(),
            repository: repository.clone(),
            version: version.clone(),
            set,
        };
        helm_chart.validate().context(ValidationSnafu {
            message: "Invalid helm chart specification",
        })?;
        if repository.is_none() && !chart.starts_with("oci://") {
            let mut errors = ValidationErrors::new();
            errors.add("repository", ValidationError::new("repository_required"));
            errors.add("chart", ValidationError::new("missing_oci_prefix"));
            return Err(SandcastleProjectError::Validation {
                message: "Repository is required for non-OCI charts".to_string(),
                source: errors,
                backtrace: Backtrace::capture(),
            });
        }
        Ok(helm_chart)
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn release_name(&self) -> &str {
        &self.release_name
    }

    pub fn chart(&self) -> &str {
        &self.chart
    }

    pub fn repository(&self) -> Option<&str> {
        self.repository.as_deref()
    }

    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    pub fn set(&self) -> Option<&[HelmChartResourceSet]> {
        self.set.as_deref()
    }
}

#[derive(Clone, Debug, Validate)]
pub struct SandcastleProjectDestination {
    #[validate(custom(function = "validate_k8s_dns_subdomain"))]
    namespace: String,
    #[validate(url)]
    server: String,
}

impl SandcastleProjectDestination {
    pub fn try_new(namespace: String, server: String) -> Result<Self, SandcastleProjectError> {
        let destination = SandcastleProjectDestination { namespace, server };
        destination.validate().context(ValidationSnafu {
            message: "Invalid destination",
        })?;
        Ok(destination)
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn server(&self) -> &str {
        &self.server
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_successful_helm_chart_resource_spec() {
        let cases = vec![
            ("default".to_string(), "https://example.com".to_string()),
            ("1234567890".to_string(), "https://example.com".to_string()),
            (
                "my-namespace-2".to_string(),
                "http://example.com".to_string(),
            ),
        ];
        for (namespace, server) in cases {
            let project = SandcastleProjectDestination::try_new(namespace, server);
            assert!(
                project.is_ok(),
                "Failed to create project destination. Got err: {:?}",
                project.err()
            );
        }
    }

    #[test]
    fn test_invalid_helm_chart_resource_spec() {
        let cases = vec![
            ("default".to_string(), "example.com".to_string()),
            (
                "-namespace".to_string(),
                "http://my-namespace-2".to_string(),
            ),
            (
                "my-*-namespace".to_string(),
                "http://example.com".to_string(),
            ),
        ];
        for (namespace, server) in cases {
            let project = SandcastleProjectDestination::try_new(namespace, server);
            assert!(
                project.is_err(),
                "Expected error for namespace. Got ok: {:?}",
                project.ok()
            );
        }
    }

    #[test]
    fn test_successful_helm_chart_spec() {
        let cases = vec![
            (
                "default".to_string(),
                "my-release".to_string(),
                "nginx".to_string(),
                Some("https://kubernetes.github.io/ingress-nginx".to_string()),
                Some("1.0.0".to_string()),
                None,
            ),
            (
                "kube-system".to_string(),
                "test-chart".to_string(),
                "oci://my-registry.com/stable/redis".to_string(),
                None,
                Some("v2.1.0".to_string()),
                Some(vec![crate::crd::HelmChartResourceSet {
                    path: "override".to_string(),
                    value: "true".to_string(),
                }]),
            ),
        ];

        for (namespace, release_name, chart, repository, version, set) in cases {
            let helm_spec = HelmChartResourceSpec::try_new(
                namespace,
                release_name,
                chart,
                repository,
                version,
                set,
            );
            assert!(
                helm_spec.is_ok(),
                "Failed to create helm chart spec. Got err: {:?}",
                helm_spec.err()
            );
        }
    }

    #[test]
    fn test_invalid_helm_chart_spec() {
        let cases = vec![
            (
                "invalid-*-namespace".to_string(),
                "my-release".to_string(),
                "nginx".to_string(),
                None,
                Some("1.0.0".to_string()),
                None,
            ),
            (
                "".to_string(),
                "invalid-*-release".to_string(),
                "nginx".to_string(),
                None,
                Some("1.0.0".to_string()),
                None,
            ),
            (
                "default".to_string(),
                "my-release".to_string(),
                "".to_string(),
                Some("https://kubernetes.github.io/ingress-nginx".to_string()),
                None,
                None,
            ),
            (
                "default".to_string(),
                "my-release".to_string(),
                "nginx".to_string(),
                Some("https://kubernetes.github.io/ingress-nginx".to_string()),
                Some("".to_string()),
                None,
            ),
        ];

        for (namespace, release_name, chart, repository, version, set) in cases {
            let helm_spec = HelmChartResourceSpec::try_new(
                namespace,
                release_name,
                chart,
                repository,
                version,
                set,
            );
            assert!(
                helm_spec.is_err(),
                "Expected error for helm chart spec. Got ok: {:?}",
                helm_spec.ok()
            );
        }
    }

    #[test]
    fn test_successful_project_resource() {
        let helm_spec = HelmChartResourceSpec::try_new(
            "default".to_string(),
            "my-release".to_string(),
            "nginx".to_string(),
            Some("https://kubernetes.github.io/ingress-nginx".to_string()),
            Some("1.0.0".to_string()),
            None,
        )
        .unwrap();

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
            let resource = SandcastleProjectResource::try_new(name, kind);
            assert!(
                resource.is_ok(),
                "Failed to create project resource. Got err: {:?}",
                resource.err()
            );
        }
    }

    #[test]
    fn test_invalid_project_resource() {
        let helm_spec = HelmChartResourceSpec::try_new(
            "default".to_string(),
            "my-release".to_string(),
            "nginx".to_string(),
            Some("https://kubernetes.github.io/ingress-nginx".to_string()),
            Some("1.0.0".to_string()),
            None,
        )
        .unwrap();

        let cases = vec![
            (
                "invalid-*-name".to_string(),
                SandcastleProjectResourceKind::HelmChart(helm_spec.clone()),
            ),
            (
                "-invalid".to_string(),
                SandcastleProjectResourceKind::HelmChart(helm_spec.clone()),
            ),
            (
                "invalid-".to_string(),
                SandcastleProjectResourceKind::HelmChart(helm_spec.clone()),
            ),
        ];

        for (name, kind) in cases {
            let resource = SandcastleProjectResource::try_new(name, kind);
            assert!(
                resource.is_err(),
                "Expected error for project resource. Got ok: {:?}",
                resource.ok()
            );
        }
    }

    #[test]
    fn test_successful_sandcastle_project() {
        let helm_spec = HelmChartResourceSpec::try_new(
            "default".to_string(),
            "my-release".to_string(),
            "nginx".to_string(),
            Some("https://kubernetes.github.io/ingress-nginx".to_string()),
            Some("1.0.0".to_string()),
            None,
        )
        .unwrap();

        let resource = SandcastleProjectResource::try_new(
            "web-server".to_string(),
            SandcastleProjectResourceKind::HelmChart(helm_spec),
        )
        .unwrap();

        let destination = SandcastleProjectDestination::try_new(
            "default".to_string(),
            "https://kubernetes.example.com".to_string(),
        )
        .unwrap();

        let project = SandcastleProject::try_new(vec![resource], destination);
        assert!(
            project.is_ok(),
            "Failed to create sandcastle project. Got err: {:?}",
            project.err()
        );
    }

    #[test]
    fn test_try_from_spec() {
        let spec = crate::crd::SandcastleProjectSpec {
            resources: vec![crate::crd::SandcastleProjectResource {
                name: "web-server".to_string(),
                kind: crate::crd::SandcastleProjectResourceKind::HelmChart(
                    crate::crd::HelmChartResourceSpec {
                        chart: "ingress-nginx".to_string(),
                        repository: Some("https://kubernetes.github.io/ingress-nginx".to_string()),
                        version: Some("4.8.0".to_string()),
                        set: None,
                    },
                ),
            }],
            destination: crate::crd::SandcastleProjectDestination {
                namespace: "default".to_string(),
                server: "https://kubernetes.example.com".to_string(),
            },
        };

        let project = SandcastleProject::try_from(spec);
        assert!(
            project.is_ok(),
            "Failed to convert from spec. Got err: {:?}",
            project.err()
        );

        let project = project.unwrap();
        assert_eq!(project.resources().len(), 1);
        assert_eq!(project.resources()[0].name(), "web-server");
        assert_eq!(project.destination().namespace(), "default");
        assert_eq!(
            project.destination().server(),
            "https://kubernetes.example.com"
        );
    }

    #[test]
    fn test_missing_oci_prefix() {
        let helm_spec = HelmChartResourceSpec::try_new(
            "default".to_string(),
            "my-release".to_string(),
            "nginx".to_string(),
            None,
            Some("1.0.0".to_string()),
            None,
        );
        assert!(
            helm_spec.is_err(),
            "Expected error for helm chart spec. Got ok: {:?}",
            helm_spec.ok()
        );
        let err = helm_spec.err().unwrap();
        match err {
            SandcastleProjectError::Validation { source, .. } => {
                let errors = source.errors();
                assert!(errors.contains_key("repository"));
                assert!(errors.contains_key("chart"));
            }
            _ => panic!("Expected validation error. Got: {:?}", err),
        }
    }
}

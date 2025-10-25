use std::{backtrace::Backtrace, collections::BTreeMap};

use async_trait::async_trait;
use axum::http::StatusCode;
use kube::{
    Api, Client, ResourceExt,
    api::{Patch, PatchParams, PostParams},
};
use sandcastle_external_crds::argocd::application::Application;
use snafu::ResultExt;

use crate::{
    domain::environment::{
        models::CreateOrUpdateArgocdApplicationRequest, ports::GitOpsPlatformService,
    },
    error::{KubeSnafu, SandcastleError, SerdeSnafu},
};

#[derive(Clone)]
pub struct ArgoCD {
    kube_client: Client,
    argocd_namespace: String,
}

impl ArgoCD {
    pub fn new(kube_client: Client, argocd_namespace: String) -> Self {
        Self {
            kube_client,
            argocd_namespace,
        }
    }
}

#[async_trait]
impl GitOpsPlatformService for ArgoCD {
    async fn create_or_update_application(
        &self,
        request: CreateOrUpdateArgocdApplicationRequest,
    ) -> Result<(), SandcastleError> {
        for application_str in request.applications {
            let mut application: Application =
                serde_yaml::from_str(&application_str).context(SerdeSnafu {
                    message: "Failed to parse ArgoCD application into application object"
                        .to_string(),
                })?;
            if let Some(labels) = application.metadata.labels.as_mut() {
                for label in request.labels.clone() {
                    labels.insert(label.0, label.1);
                }
            } else {
                application.metadata.labels = Some(BTreeMap::from_iter(request.labels.clone()));
            }
            let application_name = application.name_any();
            let application_namespace = application
                .metadata
                .namespace
                .clone()
                .unwrap_or(self.argocd_namespace.clone());
            let argocd_applications =
                Api::<Application>::namespaced(self.kube_client.clone(), &application_namespace);
            match argocd_applications.get(&application_name).await {
                Ok(application) => {
                    let pp = PatchParams::apply("sandcastle");
                    let patch = Patch::Apply(&application);
                    argocd_applications
                        .patch(&application_name, &pp, &patch)
                        .await
                        .context(KubeSnafu {
                            message: format!(
                                "Failed to patch ArgoCD application {application_name}"
                            ),
                        })?;
                }
                Err(e) => match &e {
                    kube::Error::Api(api_error) => {
                        if api_error.code == StatusCode::NOT_FOUND {
                            let pp = PostParams::default();
                            argocd_applications
                                .create(&pp, &application)
                                .await
                                .context(KubeSnafu {
                                    message: format!(
                                        "Failed to create ArgoCD application {application_name}"
                                    ),
                                })?;
                        } else {
                            return Err(SandcastleError::Kube {
                                message: "Failed to get ArgoCD application".to_string(),
                                source: e,
                                backtrace: Backtrace::capture(),
                            });
                        }
                    }
                    _ => {
                        return Err(SandcastleError::Kube {
                            message: "Failed to get ArgoCD application".to_string(),
                            source: e,
                            backtrace: Backtrace::capture(),
                        });
                    }
                },
            }
        }
        Ok(())
    }

    async fn delete_application(&self, applications: &[String]) -> Result<(), SandcastleError> {
        Ok(())
    }
}

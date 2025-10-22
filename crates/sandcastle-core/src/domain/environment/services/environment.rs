use async_trait::async_trait;
use tracing::{info, instrument};

use crate::{
    domain::environment::{
        models::{
            CreateOrUpdateArgocdApplicationAction, CreateOrUpdateArgocdApplicationRequest,
            DeleteArgocdApplicationAction, ReconcileContext,
        },
        ports::{GitOpsPlatformService, Reconcile},
    },
    error::SandcastleError,
};

#[async_trait]
impl Reconcile for CreateOrUpdateArgocdApplicationAction {
    #[instrument(skip(self, context))]
    async fn reconcile(&self, context: ReconcileContext) -> Result<(), SandcastleError> {
        info!("Creating or updating ArgoCD application");

        context
            .gitops_platform_service
            .create_or_update_application(CreateOrUpdateArgocdApplicationRequest {
                applications: self.applications.clone(),
                labels: context.labels(),
            })
            .await?;

        info!("Successfully created or updated ArgoCD application");

        Ok(())
    }
}

#[async_trait]
impl Reconcile for DeleteArgocdApplicationAction {
    #[instrument(skip(self, context))]
    async fn reconcile(&self, context: ReconcileContext) -> Result<(), SandcastleError> {
        info!("Deleting ArgoCD application");

        context
            .gitops_platform_service
            .delete_application(&self.applications)
            .await?;

        info!("Successfully deleted ArgoCD application");

        Ok(())
    }
}

use tracing::instrument;

use crate::{
    domain::{
        environment::{
            models::{
                CreateOrUpdateArgocdApplicationAction, DeleteArgocdApplicationAction,
                ReconcileContext,
            },
            ports::Reconcile,
        },
        vcs::ports::VCService,
    },
    error::SandcastleError,
};

impl<VCS: VCService> Reconcile<VCS> for CreateOrUpdateArgocdApplicationAction {
    #[instrument(skip(self, context))]
    async fn reconcile(&self, context: ReconcileContext<VCS>) -> Result<(), SandcastleError> {
        Ok(())
    }
}

impl<VCS: VCService> Reconcile<VCS> for DeleteArgocdApplicationAction {
    #[instrument(skip(self, context))]
    async fn reconcile(&self, context: ReconcileContext<VCS>) -> Result<(), SandcastleError> {
        Ok(())
    }
}

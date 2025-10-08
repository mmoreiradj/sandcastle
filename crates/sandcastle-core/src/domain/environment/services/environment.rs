use async_trait::async_trait;
use tracing::instrument;

use crate::{
    domain::environment::{
        models::{
            CreateOrUpdateArgocdApplicationAction, DeleteArgocdApplicationAction, ReconcileContext,
        },
        ports::Reconcile,
    },
    error::SandcastleError,
};

#[async_trait]
impl Reconcile for CreateOrUpdateArgocdApplicationAction {
    #[instrument(skip(self, context))]
    async fn reconcile(&self, context: ReconcileContext) -> Result<(), SandcastleError> {
        Ok(())
    }
}

#[async_trait]
impl Reconcile for DeleteArgocdApplicationAction {
    #[instrument(skip(self, context))]
    async fn reconcile(&self, context: ReconcileContext) -> Result<(), SandcastleError> {
        Ok(())
    }
}

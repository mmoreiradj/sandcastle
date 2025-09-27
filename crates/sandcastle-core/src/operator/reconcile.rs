use std::sync::Arc;
use std::time::Duration;

use kube::ResourceExt;
use kube::runtime::controller::Action;
use snafu::ResultExt;
use validator::Validate;

use crate::crd::SandcastleProject;
use crate::error::ValidationSnafu;
use crate::{
    Result,
    operator::{Context, Helm},
};

impl SandcastleProject {
    pub async fn reconcile<HELM: Helm>(&self, _context: Arc<Context<HELM>>) -> Result<Action> {
        // Validate the spec
        self.spec.validate().context(ValidationSnafu {
            message: "Invalid sandcastle project spec",
        })?;

        // TODO: Implement reconcile logic here
        tracing::info!("Reconciling SandcastleProject: {}", self.name_any());

        Ok(Action::requeue(Duration::from_secs(300)))
    }

    pub async fn cleanup<HELM: Helm>(&self, _context: Arc<Context<HELM>>) -> Result<Action> {
        // TODO: Implement cleanup logic here
        tracing::info!("Cleaning up SandcastleProject: {}", self.name_any());

        Ok(Action::await_change())
    }
}

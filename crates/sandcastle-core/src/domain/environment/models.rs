use sandcastle_external_crds::argocd::application::Application;

use super::ports::Reconcile;
use crate::{domain::vcs::ports::VCService, error::SandcastleError};

pub struct ReconcileContext<VCS: VCService> {
    pub vcs: VCS,
}

impl<VCS: VCService> ReconcileContext<VCS> {
    pub fn new(vcs: VCS) -> Self {
        Self { vcs }
    }
}

/// Action to create or update an Argocd Application
#[derive(Debug, Clone)]
pub struct CreateOrUpdateArgocdApplicationAction {
    /// The Argocd Application to create or update
    pub application: Application,
}

/// Action to delete an Argocd Application
#[derive(Debug, Clone)]
pub struct DeleteArgocdApplicationAction {
    /// The Argocd Application to delete
    pub application: Application,
}

#[derive(Debug, Clone)]
pub enum ReconcileActions {
    CreateOrUpdateArgocdApplication(CreateOrUpdateArgocdApplicationAction),
    DeleteArgocdApplication(DeleteArgocdApplicationAction),
}

impl ReconcileActions {
    pub async fn reconcile<VCS: VCService>(&self, context: ReconcileContext<VCS>) -> Result<(), SandcastleError> {
        match self {
            ReconcileActions::CreateOrUpdateArgocdApplication(action) => action.reconcile(context).await,
            ReconcileActions::DeleteArgocdApplication(action) => action.reconcile(context).await,
        }
    }
}

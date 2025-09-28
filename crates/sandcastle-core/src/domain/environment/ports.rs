use crate::{
    domain::{environment::models::ReconcileContext, vcs::ports::VCService}, error::SandcastleError,
};

/// Reconcile the environment
pub trait Reconcile<VCS: VCService> {
    fn reconcile(
        &self,
        context: ReconcileContext<VCS>,
    ) -> impl Future<Output = Result<(), SandcastleError>> + Send;
}

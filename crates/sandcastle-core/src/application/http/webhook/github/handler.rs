use axum::extract::{Json, State};
use axum::http::{HeaderName, HeaderValue};
use axum_extra::{
    TypedHeader,
    headers::{self, Header},
    routing::TypedPath,
};
use axum_macros::FromRequestParts;
use octocrab::models::webhook_events::{WebhookEvent, WebhookEventType};
use sandcastle_utils::declare_header;
use serde::Deserialize;
use serde_json::Value;
use tracing::{error, info, instrument};

use crate::application::ApplicationState;
use crate::application::reconciliation::ReconciliationService;
use crate::domain::environment::models::ReconcileContext;
use crate::domain::environment::services::VCS;
use crate::domain::repositories::ports::RepositoryConfigurationService;

#[derive(TypedPath, Deserialize)]
#[typed_path("/api/v1/github/webhook")]
pub struct HandleWebhookRoute;

declare_header!("x-github-delivery" => GithubDelivery);
declare_header!("x-hub-signature" => HubSignature);
declare_header!("x-hub-signature-256" => HubSignature256);
declare_header!("x-github-event" => GithubWebhookEventType: serde WebhookEventType);
declare_header!("x-github-hook-id" => GithubHookId: u64);
declare_header!("x-github-hook-installation-target-id" => GithubHookInstallationTargetId: u64);
declare_header!("x-github-hook-installation-target-type" => GithubHookInstallationTargetType);

#[derive(FromRequestParts)]
pub struct GithubWebhookHeaders {
    #[from_request(via(TypedHeader))]
    pub delivery: GithubDelivery,
    #[from_request(via(TypedHeader))]
    pub signature: HubSignature,
    #[from_request(via(TypedHeader))]
    pub signature_256: HubSignature256,
    #[from_request(via(TypedHeader))]
    pub event: GithubWebhookEventType,
    #[from_request(via(TypedHeader))]
    pub hook_id: GithubHookId,
    #[from_request(via(TypedHeader))]
    pub installation_target_id: GithubHookInstallationTargetId,
    #[from_request(via(TypedHeader))]
    pub installation_target_type: GithubHookInstallationTargetType,
}

/// Handle a GitHub webhook.
#[axum_macros::debug_handler]
#[instrument(skip(headers, state, payload), fields(hook_id = %headers.hook_id.0))]
pub async fn handle_webhook(
    _: HandleWebhookRoute,
    headers: GithubWebhookHeaders,
    State(state): State<ApplicationState>,
    Json(payload): Json<Value>,
) -> () {
    info!(
        delivery_id = %headers.delivery.0,
        hook_id = %headers.hook_id.0,
        installation_target_id = %headers.installation_target_id.0,
        installation_target_type = %headers.installation_target_type.0,
        "Received GitHub Event"
    );

    let event_header = headers.event.into_inner();
    let event_header_str = match serde_json::to_string(&event_header) {
        Ok(s) => s,
        Err(e) => {
            error!(error = %e, "Failed to serialize event header");
            return;
        }
    };

    let webhook_event =
        match WebhookEvent::try_from_header_and_body(&event_header_str, &payload.to_string()) {
            Ok(event) => event,
            Err(e) => {
                error!(error = %e, "Failed to parse webhook event");
                return;
            }
        };

    let event_payload = match event_header.parse_specific_payload(payload) {
        Ok(payload) => payload,
        Err(e) => {
            error!(error = %e, "Failed to parse event payload");
            return;
        }
    };

    let repository = match webhook_event.repository.clone().map(|r| r.html_url).flatten() {
        Some(repository) => repository.to_string(),
        None => {
            error!(
                "Repository not found in event payload, cannot infer authentication configuration"
            );
            return;
        }
    };

    let repository_configuration = match state
        .repository_configuration_service
        .get_repository_configuration(&repository)
        .await
    {
        Ok(Some(configuration)) => configuration,
        Ok(None) => {
            error!("Repository configuration not found, is the repository configured ?");
            return;
        }
        Err(e) => {
            error!(error = %e, "Failed to get repository configuration");
            return;
        }
    };

    let vcs_service = match VCS::try_from(repository_configuration) {
        Ok(vcs_service) => vcs_service,
        Err(e) => {
            error!(error = %e, "Failed to get VCS service");
            return;
        }
    };

    let context = match ReconcileContext::from_github_event(
        headers.hook_id.0.to_string(),
        webhook_event,
        event_payload,
        vcs_service,
    )
    .await
    {
        Ok(Some(ctx)) => ctx,
        Ok(None) => {
            info!("Event is not relevant, skipping reconciliation");
            return;
        }
        Err(e) => {
            error!(error = %e, "Failed to create reconcile context");
            return;
        }
    };

    if let Err(e) = ReconciliationService::reconcile(context).await {
        error!(error = %e, "Reconciliation failed");
    }
}

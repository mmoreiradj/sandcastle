use axum::extract::Json;
use axum::http::{HeaderName, HeaderValue};
use axum_extra::{
    TypedHeader,
    headers::{self, Header},
    routing::TypedPath,
};
use axum_macros::FromRequestParts;
use octocrab::models::webhook_events::{WebhookEvent, WebhookEventPayload, WebhookEventType};
use sandcastle_utils::declare_header;
use serde::Deserialize;
use serde_json::Value;
use tracing::info;

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
pub async fn handle_webhook(
    _: HandleWebhookRoute,
    headers: GithubWebhookHeaders,
    Json(payload): Json<Value>,
) -> () {
    println!("Delivery ID: {}", headers.delivery.0);
    println!("Hook ID: {}", headers.hook_id.0);
    println!(
        "Installation Target ID: {}",
        headers.installation_target_id.0
    );
    println!(
        "Installation Target Type: {}",
        headers.installation_target_type.0
    );
    println!("Signature: {}", headers.signature.0);
    println!("Signature 256: {}", headers.signature_256.0);

    println!(
        "Payload: {}",
        serde_json::to_string_pretty(&payload).unwrap()
    );

    let event_header = headers.event.into_inner();
    let event_header_str = serde_json::to_string(&event_header).unwrap();
    let webhook_event =
        WebhookEvent::try_from_header_and_body(&event_header_str, &payload.to_string()).unwrap();
    let event_payload = event_header.parse_specific_payload(payload).unwrap();
    match event_payload {
        WebhookEventPayload::IssueComment(payload) => {
            tracing::info!("repository: {:?}", webhook_event.repository);
            tracing::info!("sender: {:?}", webhook_event.sender);
            tracing::info!("installation: {:?}", webhook_event.installation);
        }
        _ => {
            info!("received unhandled event {:?}", event_payload);
        }
    }
}

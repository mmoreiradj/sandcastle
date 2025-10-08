use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};

pub mod webhook;

/**
 * Start the HTTP server
 */
pub async fn start() {
    let _guard = init_tracing_opentelemetry::TracingConfig::development()
        .init_subscriber()
        .unwrap();
    let router = webhook::router()
        .layer(OtelInResponseLayer)
        .layer(OtelAxumLayer::default());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

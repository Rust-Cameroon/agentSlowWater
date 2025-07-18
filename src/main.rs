use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    routing::post,
    Router,
    response::{IntoResponse, Response},
    body::{Body, to_bytes},
};

use tokio::sync::mpsc;
use tracing_subscriber;
use ci_cd_optimizer::errors::AppError;
use ci_cd_optimizer::perceiver::webhook::WebhookProcessor;
use ci_cd_optimizer::runner::agent::Agent;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Validate environment
    validate_secrets()?;

    tracing_subscriber::fmt::init();

    // Create event channel
    let (tx, rx) = mpsc::channel(100);

    // Clone tx for webhook handlers
    let github_tx = tx.clone();
    let gitlab_tx = tx;

    // Start agent
    tokio::spawn(async move {
        let mut agent = Agent::new(rx);
        agent.run().await;
    });

    // Configure routes
    let app = Router::new()
        .route("/github/webhook", post(handle_github_webhook))
        .route("/gitlab/webhook", post(handle_gitlab_webhook));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn validate_secrets() -> Result<(), AppError> {
    if std::env::var("GITHUB_WEBHOOK_SECRET").is_err() {
        return Err(AppError::ConfigError(
            "GITHUB_WEBHOOK_SECRET environment variable is required".into(),
        ));
    }

    if std::env::var("GITLAB_WEBHOOK_TOKEN").is_err() {
        return Err(AppError::ConfigError(
            "GITLAB_WEBHOOK_TOKEN environment variable is required".into(),
        ));
    }

    Ok(())
}

async fn handle_github_webhook(
    headers: HeaderMap,
    request: Request,
    tx: mpsc::Sender<ci_cd_optimizer::perceiver::event::NormalizedEvent>,
) -> Result<Response, AppError> {
    let bytes = to_bytes(request.into_body(), usize::MAX)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;
    let event = WebhookProcessor::process_github(&headers, &bytes).await?;
    tx.send(event).await
        .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;
    Ok(StatusCode::ACCEPTED.into_response())
}

async fn handle_gitlab_webhook(
    headers: HeaderMap,
    request: Request,
    tx: mpsc::Sender<ci_cd_optimizer::perceiver::event::NormalizedEvent>,
) -> Result<Response, AppError> {
    let bytes = to_bytes(request.into_body(), usize::MAX)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;
    let event = WebhookProcessor::process_gitlab(&headers, &bytes).await?;
    tx.send(event).await
        .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;
    Ok(StatusCode::ACCEPTED.into_response())
}
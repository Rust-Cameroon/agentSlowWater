use axum::http::HeaderMap;
use crate::errors::AppError;
use crate::perceiver::event::NormalizedEvent;

pub enum WebhookSource {
    GitHub,
    GitLab,
}

pub struct WebhookProcessor;

impl WebhookProcessor {
    pub async fn process_github(
        headers: &HeaderMap,
        body: &[u8],
    ) -> Result<NormalizedEvent, AppError> {
        let signature = headers
            .get("X-Hub-Signature-256")
            .and_then(|v| v.to_str().ok())
            .ok_or(AppError::Unauthorized)?;

        let secret = std::env::var("GITHUB_WEBHOOK_SECRET")
            .map_err(|_| AppError::ConfigError("Missing GITHUB_WEBHOOK_SECRET".into()))?;

        crate::perceiver::github::verifier::verify_signature(body, signature, &secret)?;
        crate::perceiver::github::parser::parse_payload(body)
    }

    pub async fn process_gitlab(
        headers: &HeaderMap,
        body: &[u8],
    ) -> Result<NormalizedEvent, AppError> {
        let token = headers
            .get("X-Gitlab-Token")
            .and_then(|v| v.to_str().ok())
            .ok_or(AppError::Unauthorized)?;

        let expected_token = std::env::var("GITLAB_WEBHOOK_TOKEN")
            .map_err(|_| AppError::ConfigError("Missing GITLAB_WEBHOOK_TOKEN".into()))?;

        if token != expected_token {
            return Err(AppError::Unauthorized);
        }

        crate::perceiver::gitlab::parser::parse_payload(body)
    }
}
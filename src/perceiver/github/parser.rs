use serde::Deserialize;
use crate::errors::AppError;
use crate::perceiver::event::{EventType, NormalizedEvent, Platform};

#[derive(Deserialize)]
pub struct GitHubWorkflowJobPayload {
    #[serde(rename = "workflow_job")]
    pub workflow_job: GitHubWorkflowJob,
    pub repository: GitHubRepository,
}

#[derive(Deserialize)]
pub struct GitHubRepository {
    pub full_name: String,
}

#[derive(Deserialize)]
pub struct GitHubWorkflowJob {
    pub id: String,
    pub run_id: String,
    pub run_attempt: Option<u32>,
    pub status: String,
    pub conclusion: Option<String>,
    pub logs_url: String,
    pub head_sha: String,
}

pub fn parse_payload(payload: &[u8]) -> Result<NormalizedEvent, AppError> {
    let payload: GitHubWorkflowJobPayload = serde_json::from_slice(payload)
        .map_err(|e| AppError::BadRequest(format!("Invalid payload: {}", e)))?;

    let event_type = match payload.workflow_job.status.as_str() {
        "queued" | "in_progress" => EventType::JobStarted,
        "completed" => match payload.workflow_job.conclusion.as_deref() {
            Some("success") => EventType::JobSucceeded,
            Some("failure") => EventType::JobFailed,
            Some("cancelled") => EventType::JobFailed,
            _ => EventType::JobFailed,
        },
        _ => EventType::JobFailed,
    };

    let mut event = NormalizedEvent::new(
        Platform::GitHub,
        payload.workflow_job.run_id.clone(),
        Some(payload.workflow_job.id.clone()),
        event_type,
        Some(payload.workflow_job.logs_url.clone()),
    );

    event.metadata.insert("repository".into(), payload.repository.full_name.clone());
    event.metadata.insert("commit_sha".into(), payload.workflow_job.head_sha.clone());
    if let Some(attempt) = payload.workflow_job.run_attempt {
        event.metadata.insert("run_attempt".into(), attempt.to_string());
    }

    Ok(event)
}
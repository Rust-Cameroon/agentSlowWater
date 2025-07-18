use serde::Deserialize;
use crate::errors::AppError;
use crate::perceiver::event::{EventType, NormalizedEvent, Platform};

#[derive(Deserialize)]
pub struct GitLabJobPayload {
    #[serde(rename = "object_attributes")]
    pub object_attributes: GitLabJobAttributes,
    pub project: GitLabProject,
    pub commit: GitLabCommit,
}

#[derive(Deserialize)]
pub struct GitLabProject {
    pub web_url: String,
    pub path_with_namespace: String,
}

#[derive(Deserialize)]
pub struct GitLabCommit {
    pub id: String,
}

#[derive(Deserialize)]
pub struct GitLabJobAttributes {
    pub id: String,
    pub pipeline_id: String,
    pub status: String,
    pub name: String,
    pub stage: String,
}

pub fn parse_payload(payload: &[u8]) -> Result<NormalizedEvent, AppError> {
    let payload: GitLabJobPayload = serde_json::from_slice(payload)
        .map_err(|e| AppError::BadRequest(format!("Invalid GitLab payload: {}", e)))?;

    let event_type = match payload.object_attributes.status.as_str() {
        "pending" | "running" => EventType::JobStarted,
        "success" => EventType::JobSucceeded,
        "failed" => EventType::JobFailed,
        _ => EventType::JobFailed,
    };

    let logs_uri = format!(
        "{}/-/jobs/{}",
        payload.project.web_url,
        payload.object_attributes.id
    );

    let mut event = NormalizedEvent::new(
        Platform::GitLab,
        payload.object_attributes.pipeline_id.clone(),
        Some(payload.object_attributes.id.clone()),
        event_type,
        Some(logs_uri),
    );

    event.metadata.insert("project".into(), payload.project.path_with_namespace.clone());
    event.metadata.insert("commit_sha".into(), payload.commit.id.clone());
    event.metadata.insert("job_name".into(), payload.object_attributes.name.clone());
    event.metadata.insert("stage".into(), payload.object_attributes.stage.clone());

    Ok(event)
}

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Platform {
    GitHub,
    GitLab,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    JobStarted,
    JobSucceeded,
    JobFailed,
    PipelineCompleted,
    TestFailure,
    DependencyIssue,
}

#[derive(Debug, Clone)]
pub struct NormalizedEvent {
    pub platform: Platform,
    pub platform_id: String, // GitHub: <repo>#<run_id>, GitLab: <project>#<pipeline_id>
    pub pipeline_id: String,
    pub job_id: Option<String>,
    pub event_type: EventType,
    pub logs_uri: Option<String>,
    pub metadata: HashMap<String, String>,
}

impl NormalizedEvent {
    pub fn new(
        platform: Platform,
        pipeline_id: String,
        job_id: Option<String>,
        event_type: EventType,
        logs_uri: Option<String>,
    ) -> Self {
        let platform_id = match platform {
            Platform::GitHub => format!("github#{}", pipeline_id),
            Platform::GitLab => format!("gitlab#{}", pipeline_id),
        };

        Self {
            platform,
            platform_id,
            pipeline_id,
            job_id,
            event_type,
            logs_uri,
            metadata: HashMap::new(),
        }
    }
}

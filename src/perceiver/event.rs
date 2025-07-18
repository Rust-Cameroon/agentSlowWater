use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Platform {
    GitHub,
    GitLab,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    /// A job has started execution (e.g., GitHub `workflow_job`, GitLab `job`).
    JobStarted,
    /// A job has successfully completed.
    JobSucceeded,
    /// A job has failed.
    JobFailed,
    /// The entire pipeline has completed successfully.
    PipelineCompleted,
    /// A test failed within a job (e.g., `check_suite` failure).
    TestFailure,
    /// A failure related to missing or uncached dependencies.
    DependencyIssue,
    /// The entire pipeline has encountered a critical error and did not complete.
    PipelineErrored,
    /// An unrecognized or unclassified event type.
    Unknown,
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
    /// Raw event name/type as received (e.g., "workflow_run", "check_suite", "pipeline")
    pub raw_event_type: Option<String>,
    /// Optional trigger info (e.g., "push", "merge_request")
    pub trigger_source: Option<String>,
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
            raw_event_type: None,
            trigger_source: None,
        }
    }
}

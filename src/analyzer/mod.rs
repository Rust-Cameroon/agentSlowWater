use crate::perceiver::event::{EventType, NormalizedEvent};
use crate::errors::AppError;

pub mod diagnosis;
pub mod log_parser;

pub async fn analyze_event(
    event: &NormalizedEvent,
    config: &crate::config::Config,
) -> Result<Vec<diagnosis::Diagnosis>, AppError> {
    let mut diagnoses = Vec::new();

    if let Some(logs_uri) = &event.logs_uri {
        let log_lines = log_parser::parse_logs(logs_uri).await?;

        if let EventType::JobFailed = event.event_type {
            if let Some((test_name, reason)) = detect_flaky_tests(&log_lines) {
                diagnoses.push(diagnosis::Diagnosis::FlakyTest {
                    test_name,
                    reason,
                });
            }
        }
    }

    Ok(diagnoses)
}

fn detect_flaky_tests(log_lines: &[String]) -> Option<(String, String)> {
    // Implement actual flaky test detection
    None
}
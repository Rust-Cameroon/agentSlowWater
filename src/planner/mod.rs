use crate::analyzer::diagnosis::Diagnosis;
use crate::perceiver::event::NormalizedEvent;
use crate::config::Config;
use crate::errors::AppError;
use crate::planner::action_plan::ActionPlan;

pub mod action_plan;
pub mod rules;

pub async fn plan_actions(
    event: &NormalizedEvent,
    diagnoses: &[Diagnosis],
    config: &Config,
) -> Result<Vec<ActionPlan>, AppError> {
    let mut actions = Vec::new();

    for diagnosis in diagnoses {
        match diagnosis {
            Diagnosis::FlakyTest { test_name, reason } => {
                if config.allow_flaky_retry {
                    if let Some(job_id) = &event.job_id {
                        actions.push(ActionPlan::RetryJob {
                            job_id: job_id.clone(),
                        });
                    }
                }

                let repo = event.metadata.get("repository").unwrap_or(&"<repo>".to_string());

                actions.push(ActionPlan::CommentOnPR {
                    message: format!(
                        "⚠️ Flaky test detected: `{}`\nReason: {}\nRepo: {}",
                        test_name, reason, repo
                    ),
                });
            }

            Diagnosis::LongRuntime { job_name, duration } => {
                actions.push(ActionPlan::CommentOnPR {
                    message: format!(
                        "⏱️ Job `{}` took too long ({}s). Consider caching or splitting steps.",
                        job_name, duration
                    ),
                });
            }

            Diagnosis::InefficientJobOrder { recommendation } => {
                actions.push(ActionPlan::CommentOnPR {
                    message: format!(
                        "🔀 Job ordering could be improved: {}",
                        recommendation
                    ),
                });
            }

            Diagnosis::CacheMiss { step } => {
                actions.push(ActionPlan::CommentOnPR {
                    message: format!(
                        "📦 Cache miss detected at step: `{}`. Consider persistent caching.",
                        step
                    ),
                });
            }

            Diagnosis::ConfigurationViolation { description } => {
                actions.push(ActionPlan::CommentOnPR {
                    message: format!(
                        "🚨 Config violation: {}",
                        description
                    ),
                });
            }

            Diagnosis::InfraFailure => {
                if let Some(job_id) = &event.job_id {
                    actions.push(ActionPlan::RetryJob {
                        job_id: job_id.clone(),
                    });
                }

                actions.push(ActionPlan::CommentOnPR {
                    message: "⚙️ Infrastructure failure detected. Retrying job...".to_string(),
                });
            }
        }
    }

    Ok(actions)
}

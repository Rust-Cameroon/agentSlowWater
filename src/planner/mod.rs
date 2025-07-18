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
    // TODO: Implement actual planning logic
    Ok(Vec::new())
}
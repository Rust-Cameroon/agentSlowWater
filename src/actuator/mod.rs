use crate::planner::action_plan::ActionPlan;
use crate::perceiver::event::NormalizedEvent;
use crate::errors::AppError;

pub mod executor;
pub mod notifier;

pub async fn execute_actions(
    event: &NormalizedEvent,
    actions: &[ActionPlan],
) -> Result<(), AppError> {
    // TODO: Implement actual action execution
    Ok(())
}
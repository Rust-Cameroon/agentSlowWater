use tokio::sync::mpsc;
use tracing::{info, error};
use crate::analyzer;
use crate::config;
use crate::planner;
use crate::actuator;
use crate::perceiver::event::NormalizedEvent;

pub struct Agent {
    receiver: mpsc::Receiver<NormalizedEvent>,
}

impl Agent {
    pub fn new(receiver: mpsc::Receiver<NormalizedEvent>) -> Self {
        Self { receiver }
    }

    pub async fn run(&mut self) {
        while let Some(event) = self.receiver.recv().await {
            if let Err(e) = self.process_event(event).await {
                error!("Error processing event: {}", e);
            }
        }
    }

    async fn process_event(&self, event: NormalizedEvent) -> anyhow::Result<()> {
        info!("Processing event: {:?}", event);

        // Load config for this repository
        let config = config::loader::load_for_event(&event).await?;

        // Analyze event
        let diagnoses = analyzer::analyze_event(&event, &config).await?;

        // Plan actions
        let actions = planner::plan_actions(&event, &diagnoses, &config).await?;

        // Execute actions
        actuator::execute_actions(&event, &actions).await?;

        Ok(())
    }
}
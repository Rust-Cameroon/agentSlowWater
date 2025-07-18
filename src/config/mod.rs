// src/config/mod.rs
pub mod loader;  // This exposes the loader submodule

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub allow_flaky_retry: bool,
    pub max_job_duration: u64,
    // Add other config fields
}

impl Default for Config {
    fn default() -> Self {
        Self {
            allow_flaky_retry: true,
            max_job_duration: 3600,
        }
    }
}
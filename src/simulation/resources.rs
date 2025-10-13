//! Shared resources and world-level data structures.

use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct SimulationConfig {
    pub tick_duration: Duration,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            tick_duration: Duration::from_secs(1),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldTime {
    pub tick: u64,
}

impl Default for WorldTime {
    fn default() -> Self {
        Self { tick: 0 }
    }
}

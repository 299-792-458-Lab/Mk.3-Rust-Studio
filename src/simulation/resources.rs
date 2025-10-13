//! Shared resources and world-level data structures.

use std::time::Duration;

use bevy_ecs::prelude::Resource;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Resource)]
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

#[derive(Debug, Clone, Resource, Serialize, Deserialize)]
pub struct WorldTime {
    pub tick: u64,
}

impl Default for WorldTime {
    fn default() -> Self {
        Self { tick: 0 }
    }
}

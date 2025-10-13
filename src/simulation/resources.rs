//! Shared resources and world-level data structures.

use std::time::Duration;

use bevy_ecs::prelude::Resource;

#[derive(Debug, Resource)]
pub struct WorldMetrics {
    pub economy: f32,
    pub satisfaction: f32,
    pub security: f32,
}

impl Default for WorldMetrics {
    fn default() -> Self {
        Self {
            economy: 50.0,
            satisfaction: 50.0,
            security: 80.0, // Start with high security
        }
    }
}

#[derive(Debug, Resource)]
pub struct DeltaTime(pub f32);

impl Default for DeltaTime {
    fn default() -> Self {
        Self(1.0)
    }
}

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

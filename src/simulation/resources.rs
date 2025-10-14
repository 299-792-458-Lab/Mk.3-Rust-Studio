//! Shared resources and world-level data structures.

use std::time::Duration;

use crate::simulation::Nation;
use bevy_ecs::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Resource, Serialize, Deserialize)]
pub struct NationMetrics {
    pub economy: f32,      // 경제
    pub science: f32,      // 과학
    pub culture: f32,      // 문화
    pub diplomacy: f32,    // 외교
    pub religion: f32,     // 종교
    pub military: f32,
    pub territory: f32,
    pub is_destroyed: bool,
}

impl Default for NationMetrics {
    fn default() -> Self {
        Self {
            economy: 50.0,
            science: 20.0,
            culture: 30.0,
            diplomacy: 30.0,
            religion: 25.0,
            military: 20.0,
            territory: 33.33,
            is_destroyed: false,
        }
    }
}

#[derive(Debug, Resource, Clone, serde::Serialize, serde::Deserialize)]
pub struct AllNationMetrics(pub HashMap<Nation, NationMetrics>);

impl Default for AllNationMetrics {
    fn default() -> Self {
        let mut metrics = HashMap::new();
        metrics.insert(Nation::Tera, NationMetrics::default());
        metrics.insert(Nation::Sora, NationMetrics::default());
        metrics.insert(Nation::Aqua, NationMetrics::default());
        Self(metrics)
    }
}

#[derive(Debug, Resource)]
pub struct DeltaTime(pub f32);

impl Default for DeltaTime {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Debug, Clone, Resource)]
pub struct SimulationConfig {
    pub tick_duration: Duration,
    pub grid_radius: i32,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            tick_duration: Duration::from_secs(1),
            grid_radius: 5,
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

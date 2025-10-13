//! Shared observer snapshot structures exported via the API.

use crate::simulation::{AllNationMetrics, BehaviorState, Biome, Faction, Nation, WorldEvent, HexGrid};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct EntitySnapshot {
    pub id: u64,
    pub name: String,
    pub faction: Faction,
    pub faction_label: String,
    pub biome: Biome,
    pub biome_label: String,
    pub behavior_state: BehaviorState,
    pub behavior_label: String,
    pub currency: f32,
    pub wealth: f32,
    pub fame: f32,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ObserverSnapshot {
    pub tick: u64,
    pub epoch: String,
    pub season: String,
    pub all_metrics: AllNationMetrics,
    pub grid: HexGrid,
    pub entities: Vec<EntitySnapshot>,
    pub events: Vec<WorldEvent>,
}

impl ObserverSnapshot {
    pub fn new() -> Self {
        Self {
            tick: 0,
            epoch: "새벽".to_string(),
            season: "꽃피움 계절".to_string(),
            all_metrics: AllNationMetrics::default(),
            grid: HexGrid::default(),
            entities: Vec::new(),
            events: Vec::new(),
        }
    }

    pub fn update(
        &mut self,
        tick: u64,
        epoch: String,
        season: String,
        metrics: &AllNationMetrics,
        grid: &HexGrid,
        entities: Vec<EntitySnapshot>,
        events: Vec<WorldEvent>,
    ) {
        self.tick = tick;
        self.epoch = epoch;
        self.season = season;
        self.all_metrics = metrics.clone();
        self.grid = grid.clone();
        self.entities = entities;
        self.events = events;
    }
}

impl Default for ObserverSnapshot {
    fn default() -> Self {
        Self::new()
    }
}

//! Shared observer snapshot structures exported via the API.

use serde::Serialize;

use crate::simulation::{BehaviorState, Biome, Faction, WorldEvent};

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

#[derive(Debug, Clone, Serialize)]
pub struct ObserverSnapshot {
    pub tick: u64,
    pub epoch: String,
    pub season: String,
    pub economy: f32,
    pub satisfaction: f32,
    pub security: f32,
    pub entities: Vec<EntitySnapshot>,
    pub events: Vec<WorldEvent>,
}

impl ObserverSnapshot {
    pub fn new() -> Self {
        Self {
            tick: 0,
            epoch: "새벽".to_string(),
            season: "꽃피움 계절".to_string(),
            economy: 50.0,
            satisfaction: 50.0,
            security: 80.0,
            entities: Vec::new(),
            events: Vec::new(),
        }
    }

    pub fn update(
        &mut self,
        tick: u64,
        epoch: String,
        season: String,
        metrics: &WorldMetrics,
        entities: Vec<EntitySnapshot>,
        events: Vec<WorldEvent>,
    ) {
        self.tick = tick;
        self.epoch = epoch;
        self.season = season;
        self.economy = metrics.economy;
        self.satisfaction = metrics.satisfaction;
        self.security = metrics.security;
        self.entities = entities;
        self.events = events;
    }
}

impl Default for ObserverSnapshot {
    fn default() -> Self {
        Self::new()
    }
}

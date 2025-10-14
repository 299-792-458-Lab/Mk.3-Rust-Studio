//! Shared observer snapshot structures exported via the API.

use crate::simulation::{AllNationMetrics, BehaviorState, Biome, Faction, WorldEvent, AxialCoord, Nation};
use serde::Serialize;
use std::collections::{HashMap, HashSet};

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
pub struct HexGridSnapshot {
    pub hexes: HashMap<AxialCoord, HexSnapshot>,
    pub radius: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct HexSnapshot {
    pub owner: Nation,
}

#[derive(Debug, Clone, Serialize)]
pub struct ObserverSnapshot {
    pub tick: u64,
    pub epoch: String,
    pub season: String,
    pub all_metrics: AllNationMetrics,
    pub grid: HexGridSnapshot,
    pub entities: Vec<EntitySnapshot>,
    pub events: Vec<WorldEvent>,
    pub combat_hexes: HashSet<AxialCoord>,
}

impl ObserverSnapshot {
    pub fn new() -> Self {
        Self {
            tick: 0,
            epoch: "새벽".to_string(),
            season: "꽃피움 계절".to_string(),
            all_metrics: AllNationMetrics::default(),
            grid: HexGridSnapshot::default(),
            entities: Vec::new(),
            events: Vec::new(),
            combat_hexes: HashSet::new(),
        }
    }

    pub fn update(
        &mut self,
        tick: u64,
        epoch: String,
        season: String,
        metrics: &AllNationMetrics,
        grid: HexGridSnapshot,
        entities: Vec<EntitySnapshot>,
        events: Vec<WorldEvent>,
        combat_hexes: HashSet<AxialCoord>,
    ) {
        self.tick = tick;
        self.epoch = epoch;
        self.season = season;
        self.all_metrics = metrics.clone();
        self.grid = grid;
        self.entities = entities;
        self.events = events;
        self.combat_hexes = combat_hexes;
    }
}

impl Default for ObserverSnapshot {
    fn default() -> Self {
        Self::new()
    }
}

//! Structured world event data and observer-facing snapshots.

use std::collections::VecDeque;

use bevy_ecs::prelude::Resource;
use serde::{Deserialize, Serialize};

use crate::simulation::{BehaviorState, Biome, Faction};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WorldEventKind {
    Trade {
        actor: EventActor,
        trade_focus: String,
        market_pressure: String,
    },
    Social {
        convener: EventActor,
        gathering_theme: String,
        cohesion_level: String,
    },
    MacroShock {
        stressor: String,
        catalyst: String,
        projected_impact: String,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Sentiment {
    Positive,
    Neutral,
    Negative,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventActor {
    pub id: u64,
    pub name: String,
    pub faction: Faction,
    pub faction_label: String,
    pub biome: Biome,
    pub biome_label: String,
    pub behavior_hint: BehaviorState,
    pub behavior_hint_label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldEvent {
    pub tick: u64,
    pub epoch: String,
    pub season: String,
    pub kind: WorldEventKind,
}

impl WorldEvent {
    pub fn category(&self) -> &'static str {
        match &self.kind {
            WorldEventKind::Trade { .. } => "무역",
            WorldEventKind::Social { .. } => "사회",
            WorldEventKind::MacroShock { .. } => "거시충격",
        }
    }

    pub fn sentiment(&self) -> Sentiment {
        match &self.kind {
            WorldEventKind::Trade { .. } => Sentiment::Positive,
            WorldEventKind::Social { .. } => Sentiment::Positive,
            WorldEventKind::MacroShock { .. } => Sentiment::Negative,
        }
    }

    #[allow(dead_code)]
    pub fn headline(&self) -> String {
        match &self.kind {
            WorldEventKind::Trade {
                actor,
                trade_focus,
                market_pressure,
            } => format!(
                "{} 님이 {} 거래를 조율합니다 | 압력: {}",
                actor.name, trade_focus, market_pressure
            ),
            WorldEventKind::Social {
                convener,
                gathering_theme,
                cohesion_level,
            } => format!(
                "{} 님이 \"{}\" 주제로 모임을 주관합니다 | 응집도: {}",
                convener.name, gathering_theme, cohesion_level
            ),
            WorldEventKind::MacroShock {
                stressor,
                catalyst,
                projected_impact,
            } => format!(
                "{} | 촉발 요인: {} | 영향: {}",
                stressor, catalyst, projected_impact
            ),
        }
    }

    pub fn trade(
        tick: u64,
        epoch: &str,
        season: &str,
        actor: EventActor,
        trade_focus: String,
        market_pressure: String,
    ) -> Self {
        Self {
            tick,
            epoch: epoch.to_string(),
            season: season.to_string(),
            kind: WorldEventKind::Trade {
                actor,
                trade_focus,
                market_pressure,
            },
        }
    }

    pub fn social(
        tick: u64,
        epoch: &str,
        season: &str,
        convener: EventActor,
        gathering_theme: String,
        cohesion_level: String,
    ) -> Self {
        Self {
            tick,
            epoch: epoch.to_string(),
            season: season.to_string(),
            kind: WorldEventKind::Social {
                convener,
                gathering_theme,
                cohesion_level,
            },
        }
    }

    pub fn macro_shock(
        tick: u64,
        epoch: &str,
        season: &str,
        stressor: String,
        catalyst: String,
        projected_impact: String,
    ) -> Self {
        Self {
            tick,
            epoch: epoch.to_string(),
            season: season.to_string(),
            kind: WorldEventKind::MacroShock {
                stressor,
                catalyst,
                projected_impact,
            },
        }
    }
}

#[derive(Debug, Resource)]
pub struct WorldEventLog {
    events: VecDeque<WorldEvent>,
    capacity: usize,
}

impl WorldEventLog {
    pub fn new(capacity: usize) -> Self {
        Self {
            events: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, event: WorldEvent) {
        if self.events.len() == self.capacity {
            self.events.pop_front();
        }
        self.events.push_back(event);
    }

    pub fn snapshot(&self) -> Vec<WorldEvent> {
        self.events.iter().cloned().collect()
    }
}

impl Default for WorldEventLog {
    fn default() -> Self {
        Self::new(256)
    }
}

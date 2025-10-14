use bevy_ecs::prelude::{Component, Entity, Resource};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::simulation::Nation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Component)]
pub struct AxialCoord {
    pub q: i32,
    pub r: i32,
}

impl AxialCoord {
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }

    pub fn neighbors(&self) -> [AxialCoord; 6] {
        [
            AxialCoord::new(self.q + 1, self.r),
            AxialCoord::new(self.q - 1, self.r),
            AxialCoord::new(self.q, self.r + 1),
            AxialCoord::new(self.q, self.r - 1),
            AxialCoord::new(self.q + 1, self.r - 1),
            AxialCoord::new(self.q - 1, self.r + 1),
        ]
    }
}

#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub struct Hex {
    pub owner: Nation,
}

#[derive(Debug, Clone, Resource, Serialize, Deserialize, Default)]
pub struct HexGrid {
    pub hexes: HashMap<AxialCoord, Entity>,
    pub radius: i32,
}

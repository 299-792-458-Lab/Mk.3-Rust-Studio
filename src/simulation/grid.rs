use bevy_ecs::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::simulation::Nation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AxialCoord {
    pub q: i32,
    pub r: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hex {
    pub coord: AxialCoord,
    pub owner: Nation,
}

#[derive(Debug, Clone, Resource, Serialize, Deserialize, Default)]
pub struct HexGrid {
    pub hexes: HashMap<AxialCoord, Hex>,
    pub radius: i32,
}

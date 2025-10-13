//! Trade and economy placeholder system.

use bevy_ecs::prelude::*;

use crate::simulation::{BehaviorState, Inventory};

pub fn economy_system(mut query: Query<(&BehaviorState, &mut Inventory)>) {
    for (state, mut inventory) in &mut query {
        if matches!(state, BehaviorState::Trade) {
            inventory.currency += 5.0;
        }
    }
}

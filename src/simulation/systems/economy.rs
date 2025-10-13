//! Trade and economy placeholder system.

use bevy_ecs::prelude::*;

use crate::simulation::{Behavior, BehaviorState, Inventory};

pub fn economy_system(mut query: Query<(&Behavior, &mut Inventory)>) {
    for (behavior, mut inventory) in &mut query {
        if matches!(behavior.state, BehaviorState::Trade) {
            inventory.currency += 5.0;
        }
    }
}

//! AI state transition system.

use bevy_ecs::prelude::*;

use crate::simulation::{Behavior, BehaviorState, Personality, WorldTime};

pub fn ai_state_transition_system(
    mut query: Query<(&Personality, &mut Behavior)>,
    time: Res<WorldTime>,
) {
    let _tick = time.tick;
    for (_personality, mut behavior) in &mut query {
        // Placeholder logic: cycle through states to keep entities active during development.
        behavior.state = match behavior.state {
            BehaviorState::Idle => BehaviorState::Explore,
            BehaviorState::Explore => BehaviorState::Gather,
            BehaviorState::Gather => BehaviorState::Trade,
            BehaviorState::Trade => BehaviorState::Hunt,
            BehaviorState::Hunt => BehaviorState::Rest,
            BehaviorState::Rest => BehaviorState::Idle,
        };
    }
}

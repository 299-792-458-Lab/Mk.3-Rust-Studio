//! Event logging placeholder system.

use bevy_ecs::prelude::*;
use tracing::info;

use crate::simulation::WorldTime;

pub fn logging_system(time: Res<WorldTime>) {
    info!(tick = time.tick, "simulation tick processed");
}

//! Trade and economy placeholder system.

use bevy_ecs::prelude::*;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use crate::simulation::{
    Behavior, BehaviorState, Identity, Inventory, Position, WorldMetadata, WorldTime,
};

fn season_trade_modifier(season: &str) -> f32 {
    match season {
        "꽃피움 계절" => 1.1,
        "불꽃 절정" => 1.0,
        "잿불 내림" => 0.95,
        _ => 1.0,
    }
}

fn season_gather_modifier(season: &str) -> f32 {
    match season {
        "꽃피움 계절" => 1.25,
        "불꽃 절정" => 1.05,
        "잿불 내림" => 0.9,
        _ => 1.0,
    }
}

fn segment_trade_modifier(segment: &str) -> f32 {
    match segment {
        "한낮" => 1.2,
        "해질녘" => 0.85,
        _ => 1.0,
    }
}

fn upkeep_penalty(base: f32, upkeep: f32) -> f32 {
    base * upkeep.max(0.5)
}

pub fn economy_system(
    mut query: Query<(&Identity, &Position, &Behavior, &mut Inventory)>,
    world_meta: Res<WorldMetadata>,
    time: Res<WorldTime>,
) {
    let (segment, season) = world_meta.epoch_for_tick(time.tick);

    for (identity, position, behavior, mut inventory) in &mut query {
        let biome = position.biome;
        let faction = identity.faction;

        let base_trade_yield = 6.0;
        let base_gather_value = 3.0;
        let trade_multiplier = world_meta.biome_trade_opportunity(biome)
            * world_meta.faction_trade_yield(faction)
            * season_trade_modifier(season)
            * segment_trade_modifier(segment);
        let resource_multiplier =
            world_meta.biome_resource_abundance(biome) * season_gather_modifier(season);
        let risk_factor =
            world_meta.biome_risk_factor(biome) / world_meta.faction_volatility_resistance(faction);

        let upkeep = world_meta.faction_upkeep_burden(faction);

        let mut rng = SmallRng::seed_from_u64(
            time.tick
                .wrapping_mul(131)
                .wrapping_add(identity.id * 7)
                .wrapping_mul(59),
        );

        if matches!(behavior.state, BehaviorState::Trade) {
            let volatility: f32 = rng.gen_range(-2.0..2.0) * risk_factor;
            let trade_gain =
                base_trade_yield * trade_multiplier - upkeep_penalty(0.75, upkeep) + volatility;
            inventory.currency =
                (inventory.currency + trade_gain.max(-inventory.currency)).max(0.0);
        }

        if matches!(behavior.state, BehaviorState::Gather) {
            let gather_gain = base_gather_value * resource_multiplier
                - upkeep_penalty(0.35, upkeep)
                + rng.gen_range(0.0..2.0);
            inventory.currency += gather_gain.max(0.0);
        }
    }
}

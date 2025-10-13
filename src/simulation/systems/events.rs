//! Generates structured world events tied to metadata influences.

use bevy_ecs::prelude::*;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use crate::simulation::{
    Attributes, Behavior, BehaviorState, EventActor, Inventory, Position, WorldEvent, WorldEventLog,
    WorldMetadata, WorldTime,
};

pub fn event_generation_system(
    time: Res<WorldTime>,
    world_meta: Res<WorldMetadata>,
    mut event_log: ResMut<WorldEventLog>,
    query: Query<(&crate::simulation::Identity, &Behavior, &Position, &Inventory, &Attributes)>,
) {
    let tick = time.tick;
    let (epoch, season) = world_meta.epoch_for_tick(tick);
    let mut rng = SmallRng::seed_from_u64(tick.wrapping_mul(421) + 17);

    // Trade event sampling
    let mut trade_choice: Option<(
        crate::simulation::Identity,
        Position,
        f32,
        BehaviorState,
    )> = None;
    let mut trade_count = 0;

    for (identity, behavior, position, inventory, _) in &query {
        if !matches!(behavior.state, BehaviorState::Trade) {
            continue;
        }
        trade_count += 1;
        if rng.gen_range(0..trade_count) == 0 {
            trade_choice = Some(((*identity).clone(), *position, inventory.currency, behavior.state));
        }
    }

    if let Some((identity, position, currency, behavior_state)) = trade_choice {
        let biome_profile = world_meta.biomes.get(&position.biome);
        let focus = biome_profile
            .and_then(|meta| {
                if meta.resource_profile.is_empty() {
                    None
                } else {
                    Some(
                        meta.resource_profile[rng.gen_range(0..meta.resource_profile.len())]
                            .to_string(),
                    )
                }
            })
            .unwrap_or_else(|| "General commodities".to_string());

        let pressure = world_meta
            .faction_profile(identity.faction)
            .and_then(|f| {
                if f.influence_vectors.is_empty() {
                    None
                } else {
                    Some(
                        f.influence_vectors[rng.gen_range(0..f.influence_vectors.len())]
                            .to_string(),
                    )
                }
            })
            .unwrap_or_else(|| "Local demand feedback".to_string());

        let actor = EventActor {
            id: identity.id,
            name: identity.name.clone(),
            faction: identity.faction,
            biome: position.biome,
            behavior_hint: behavior_state,
        };

        let trade_summary = format!(
            "{} brokers {} flows in {} (liquidity {:.1})",
            actor.name,
            focus,
            biome_profile
                .map(|meta| meta.label)
                .unwrap_or("Unknown exchange"),
            currency
        );

        event_log.push(WorldEvent::trade(
            tick,
            epoch,
            season,
            actor,
            trade_summary,
            pressure,
        ));
    }

    // Social event sampling (Idle or Rest)
    let mut social_choice: Option<(
        crate::simulation::Identity,
        Position,
        Attributes,
        BehaviorState,
    )> = None;
    let mut social_count = 0;

    for (identity, behavior, position, _, attributes) in &query {
        if !matches!(behavior.state, BehaviorState::Idle | BehaviorState::Rest) {
            continue;
        }
        social_count += 1;
        if rng.gen_range(0..social_count) == 0 {
            social_choice = Some(((*identity).clone(), *position, (*attributes).clone(), behavior.state));
        }
    }

    if let Some((identity, position, attributes, behavior_state)) = social_choice {
        let biome_profile = world_meta.biomes.get(&position.biome);
        let gathering_theme = biome_profile
            .and_then(|meta| {
                if meta.tensions.is_empty() {
                    None
                } else {
                    Some(
                        meta.tensions[rng.gen_range(0..meta.tensions.len())]
                            .to_string(),
                    )
                }
            })
            .unwrap_or_else(|| "Shared stories".to_string());

        let cohesion_level = if attributes.fame >= 60.0 {
            "Legendary draw"
        } else if attributes.fame >= 35.0 {
            "Strong turnout"
        } else if attributes.fame >= 15.0 {
            "Modest gathering"
        } else {
            "Intimate circle"
        }
        .to_string();

        let actor = EventActor {
            id: identity.id,
            name: identity.name.clone(),
            faction: identity.faction,
            biome: position.biome,
            behavior_hint: behavior_state,
        };

        event_log.push(WorldEvent::social(
            tick,
            epoch,
            season,
            actor,
            gathering_theme,
            cohesion_level,
        ));
    }

    // Macro shock event (pulse each tick)
    let stressors = &world_meta.economy.stressors;
    let catalysts = &world_meta.economy.catalysts;
    let circulation = &world_meta.economy.circulation_cycle;

    if !(stressors.is_empty() || catalysts.is_empty() || circulation.is_empty()) {
        let stressor = stressors[(tick as usize) % stressors.len()].to_string();
        let catalyst =
            catalysts[((tick as usize) + circulation.len()) % catalysts.len()].to_string();
        let circulation_stage =
            circulation[(tick as usize + catalysts.len()) % circulation.len()].to_string();

        let impact = format!(
            "{} strains {} while {} is underway",
            stressor, catalyst, circulation_stage
        );

        event_log.push(WorldEvent::macro_shock(
            tick,
            epoch,
            season,
            stressor,
            catalyst,
            impact,
        ));
    }
}

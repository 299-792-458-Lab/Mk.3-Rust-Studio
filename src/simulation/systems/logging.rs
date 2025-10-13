//! Event logging placeholder system with world metadata narrative hooks.

use bevy_ecs::prelude::*;
use tracing::info;

use crate::simulation::{Behavior, Identity, Position, WorldMetadata, WorldTime};

pub fn logging_system(
    time: Res<WorldTime>,
    world_meta: Res<WorldMetadata>,
    query: Query<(&Identity, &Behavior, &Position)>,
) {
    let (epoch, season) = world_meta.epoch_for_tick(time.tick);
    let catalyst_index = (time.tick as usize) % world_meta.economy.catalysts.len();
    let catalyst = world_meta.economy.catalysts[catalyst_index];
    let circulation_stage = world_meta
        .economy
        .circulation_cycle
        .get(catalyst_index % world_meta.economy.circulation_cycle.len())
        .copied()
        .unwrap_or("Balanced exchange");
    let stressor = world_meta
        .economy
        .stressors
        .get(catalyst_index % world_meta.economy.stressors.len())
        .copied()
        .unwrap_or("Stable outlook");

    let sample = query
        .iter()
        .next()
        .map(|(identity, behavior, position)| {
            let (biome_summary, resource_hint, tension_hint, description_snippet) = world_meta
                .biomes
                .get(&position.biome)
                .map(|b| {
                    let resource = b
                        .resource_profile
                        .first()
                        .copied()
                        .unwrap_or("General goods");
                    let tension = b.tensions.first().copied().unwrap_or("Quiet watch");
                    (
                        format!("{} — {}", b.label, b.epithet),
                        resource,
                        tension,
                        b.description,
                    )
                })
                .unwrap_or_else(|| {
                    (
                        "Unknown biome".to_string(),
                        "Unknown resource",
                        "Unknown tension",
                        "No records",
                    )
                });

            let faction_thread = world_meta
                .faction_profile(identity.faction)
                .map(|f| {
                    let vector = f
                        .influence_vectors
                        .first()
                        .copied()
                        .unwrap_or("Subtle influence");
                    let stronghold = f
                        .strongholds
                        .first()
                        .map(|biome| format!("{biome:?}"))
                        .unwrap_or_else(|| "No stronghold".to_string());
                    format!(
                        "{} | Doctrine: {} | Lever: {} | Base: {}",
                        f.motto, f.doctrine, vector, stronghold
                    )
                })
                .unwrap_or_else(|| "Unaligned motives".to_string());

            format!(
                "{} ({:?}) is {:?} within {} | Focus: {} | Tension: {} | Atmosphere: {} | {}",
                identity.name,
                identity.faction,
                behavior.state,
                biome_summary,
                resource_hint,
                tension_hint,
                description_snippet,
                faction_thread
            )
        })
        .unwrap_or_else(|| "No entities present".to_string());

    info!(
        tick = time.tick,
        epoch, season, catalyst, circulation_stage, stressor, sample, "world pulse"
    );
}

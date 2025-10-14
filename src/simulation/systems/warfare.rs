use bevy_ecs::prelude::*;
use crate::simulation::{
    AllNationMetrics, Nation, WorldTime, Hex,
    components::{InCombat, Combatants},
    grid::AxialCoord,
};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::collections::{HashMap, HashSet};

struct BattleRequest {
    nation_a: Nation,
    nation_b: Nation,
}

// System to clean up finished combat encounters
pub fn combat_cleanup_system(mut commands: Commands, mut query: Query<(Entity, &mut InCombat)>) {
    for (entity, mut in_combat) in query.iter_mut() {
        in_combat.ticks_remaining = in_combat.ticks_remaining.saturating_sub(1);
        if in_combat.ticks_remaining == 0 {
            commands.entity(entity).remove::<InCombat>();
            commands.entity(entity).remove::<Combatants>();
        }
    }
}

pub fn warfare_system(
    mut commands: Commands,
    mut all_metrics: ResMut<AllNationMetrics>,
    time: Res<WorldTime>,
    mut event_log: ResMut<crate::simulation::WorldEventLog>,
    world_meta: Res<crate::simulation::WorldMetadata>,
    hex_query: Query<(Entity, &Hex, &AxialCoord)>,
) {
    let mut rng = SmallRng::seed_from_u64(time.tick.wrapping_mul(257));
    let mut battle_requests = Vec::new();

    // 1. Identify potential battles
    let nations: Vec<Nation> = all_metrics.0.keys().cloned().collect();
    for i in 0..nations.len() {
        for j in (i + 1)..nations.len() {
            let nation_a_key = nations[i];
            let nation_b_key = nations[j];

            let (metrics_a, metrics_b) = (
                all_metrics.0.get(&nation_a_key).unwrap(),
                all_metrics.0.get(&nation_b_key).unwrap(),
            );

            if metrics_a.is_destroyed || metrics_b.is_destroyed {
                continue;
            }

            // 10% chance of battle each tick
            if rng.gen_bool(0.1) {
                battle_requests.push(BattleRequest { nation_a: nation_a_key, nation_b: nation_b_key });
            }
        }
    }

    let (epoch, season) = world_meta.epoch_for_tick(time.tick);

    // 2. Process battles
    for request in battle_requests {
        let (winner, loser) = {
            let metrics_a = all_metrics.0.get(&request.nation_a).unwrap();
            let metrics_b = all_metrics.0.get(&request.nation_b).unwrap();
            let military_a = metrics_a.military;
            let military_b = metrics_b.military;

            let roll_a = rng.gen_range(0.0..1.0) * military_a;
            let roll_b = rng.gen_range(0.0..1.0) * military_b;

            if roll_a > roll_b {
                (request.nation_a, request.nation_b)
            } else {
                (request.nation_b, request.nation_a)
            }
        };

        // Update metrics
        let territory_change = 0.5;
        let military_loss = 2.0;

        if let Some(winner_metrics) = all_metrics.0.get_mut(&winner) {
            winner_metrics.territory += territory_change;
            winner_metrics.military -= military_loss;
            winner_metrics.territory = winner_metrics.territory.max(0.0);
            winner_metrics.military = winner_metrics.military.max(0.0);
        }

        if let Some(loser_metrics) = all_metrics.0.get_mut(&loser) {
            loser_metrics.territory -= territory_change;
            loser_metrics.military -= military_loss;
            loser_metrics.territory = loser_metrics.territory.max(0.0);
            loser_metrics.military = loser_metrics.military.max(0.0);

            if loser_metrics.territory <= 0.0 {
                loser_metrics.is_destroyed = true;
            }
        }

        // Log the event
        event_log.push(crate::simulation::WorldEvent::warfare(
            time.tick,
            epoch,
            season,
            winner,
            loser,
            territory_change,
        ));

        // 3. Find border hexes and mark them as in combat
        let mut border_hex_entities = HashSet::new();
        let nation_hexes: HashMap<Nation, HashSet<AxialCoord>> = {
            let mut map: HashMap<Nation, HashSet<AxialCoord>> = HashMap::new();
            for (_, hex, coord) in hex_query.iter() {
                map.entry(hex.owner).or_default().insert(*coord);
            }
            map
        };

        let loser_hexes = nation_hexes.get(&loser).cloned().unwrap_or_default();

        for (entity, hex, coord) in hex_query.iter() {
            if hex.owner == winner {
                for neighbor_coord in coord.neighbors() {
                    if loser_hexes.contains(&neighbor_coord) {
                        border_hex_entities.insert(entity);
                    }
                }
            }
        }

        for entity in border_hex_entities {
            commands.entity(entity).insert((
                InCombat { ticks_remaining: 5 },
                Combatants { nation_a: winner, nation_b: loser },
            ));
        }
    }
}

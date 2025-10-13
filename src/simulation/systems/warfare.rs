// Warfare system

use bevy_ecs::prelude::*;
use crate::simulation::{AllNationMetrics, Nation, WorldTime};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

struct BattleResult {
    winner: Nation,
    loser: Nation,
}

pub fn warfare_system(
    mut all_metrics: ResMut<AllNationMetrics>,
    time: Res<WorldTime>,
) {
    let mut rng = SmallRng::seed_from_u64(time.tick.wrapping_mul(257));
    let mut battle_results = Vec::new();

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

            if rng.gen_bool(0.1) { // 10% chance of battle each tick
                let military_a = metrics_a.military;
                let military_b = metrics_b.military;

                let roll_a = rng.gen_range(0.0..1.0) * military_a;
                let roll_b = rng.gen_range(0.0..1.0) * military_b;

                if roll_a > roll_b {
                    battle_results.push(BattleResult { winner: nation_a_key, loser: nation_b_key });
                } else {
                    battle_results.push(BattleResult { winner: nation_b_key, loser: nation_a_key });
                }
            }
        }
    }

    for result in battle_results {
        let territory_change = 0.5;
        let military_loss = 2.0;

        if let Some(winner_metrics) = all_metrics.0.get_mut(&result.winner) {
            winner_metrics.territory += territory_change;
            winner_metrics.military -= military_loss;
            winner_metrics.territory = winner_metrics.territory.max(0.0);
            winner_metrics.military = winner_metrics.military.max(0.0);
        }

        if let Some(loser_metrics) = all_metrics.0.get_mut(&result.loser) {
            loser_metrics.territory -= territory_change;
            loser_metrics.military -= military_loss;
            loser_metrics.territory = loser_metrics.territory.max(0.0);
            loser_metrics.military = loser_metrics.military.max(0.0);

            if loser_metrics.territory <= 0.0 {
                loser_metrics.is_destroyed = true;
            }
        }
    }
}
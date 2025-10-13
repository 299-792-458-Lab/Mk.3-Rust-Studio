use bevy_ecs::prelude::*;
use crate::simulation::{AllNationMetrics, HexGrid, Nation};

fn distance(q1: i32, r1: i32, q2: i32, r2: i32) -> i32 {
    ((q1 - q2).abs() + (q1 + r1 - q2 - r2).abs() + (r1 - r2).abs()) / 2
}

pub fn territory_system(
    metrics: Res<AllNationMetrics>,
    mut grid: ResMut<HexGrid>,
) {
    let capitals = [
        (Nation::Tera, 0, 0),
        (Nation::Sora, -4, 0),
        (Nation::Aqua, 4, 0),
    ];

    for (_coord, hex) in grid.hexes.iter_mut() {
        let mut max_influence = -1.0;
        let mut owner = hex.owner;

        for (nation, cap_q, cap_r) in &capitals {
            let territory = metrics.0.get(nation).map_or(0.0, |m| m.territory);
            let dist = distance(hex.coord.q, hex.coord.r, *cap_q, *cap_r);

            // Avoid division by zero and handle distance 0 case
            let influence = if dist == 0 {
                f32::MAX
            } else {
                territory / (dist as f32).powi(2)
            };

            if influence > max_influence {
                max_influence = influence;
                owner = *nation;
            }
        }
        hex.owner = owner;
    }
}

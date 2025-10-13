//! Event logging placeholder system with world metadata narrative hooks.

use bevy_ecs::prelude::*;
use tracing::info;

use crate::simulation::{
    Behavior, Biome, Identity, Position, WorldMetadata, WorldTime, behavior_label, faction_label,
};

fn biome_label(meta: &WorldMetadata, biome: Biome) -> String {
    meta.biomes
        .get(&biome)
        .map(|b| b.label.to_string())
        .unwrap_or_else(|| format!("{biome:?}"))
}

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
        .unwrap_or("균형 거래");
    let stressor = world_meta
        .economy
        .stressors
        .get(catalyst_index % world_meta.economy.stressors.len())
        .copied()
        .unwrap_or("안정 국면");

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
                        .unwrap_or("일반 물자");
                    let tension = b.tensions.first().copied().unwrap_or("고요한 경계");
                    (
                        format!("{} — {}", b.label, b.epithet),
                        resource,
                        tension,
                        b.description,
                    )
                })
                .unwrap_or_else(|| {
                    (
                        "미확인 생태구역".to_string(),
                        "미확인 자원",
                        "미확인 긴장",
                        "기록 없음",
                    )
                });

            let faction_thread = world_meta
                .faction_profile(identity.faction)
                .map(|f| {
                    let vector = f
                        .influence_vectors
                        .first()
                        .copied()
                        .unwrap_or("은밀한 영향");
                    let stronghold = f
                        .strongholds
                        .first()
                        .map(|biome| biome_label(&world_meta, *biome))
                        .unwrap_or_else(|| "거점 없음".to_string());
                    format!(
                        "{} | 교리: {} | 영향 축: {} | 거점: {}",
                        f.motto, f.doctrine, vector, stronghold
                    )
                })
                .unwrap_or_else(|| "분류되지 않은 동기".to_string());

            format!(
                "{} ({}) 는 {} 상태로 {}에 있습니다 | 초점 자원: {} | 긴장 요인: {} | 분위기: {} | {}",
                identity.name,
                faction_label(identity.faction),
                behavior_label(behavior.state),
                biome_summary,
                resource_hint,
                tension_hint,
                description_snippet,
                faction_thread
            )
        })
        .unwrap_or_else(|| "관측 가능한 개체가 없습니다".to_string());

    info!(
        tick = time.tick,
        epoch, season, catalyst, circulation_stage, stressor, sample, "세계 맥동"
    );
}

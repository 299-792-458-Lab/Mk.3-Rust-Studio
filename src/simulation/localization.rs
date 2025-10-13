//! Localization helpers that convert core enums into Korean display strings.

use crate::simulation::{BehaviorState, Faction};

pub fn behavior_label(state: BehaviorState) -> &'static str {
    match state {
        BehaviorState::Idle => "휴식 대기",
        BehaviorState::Explore => "탐험",
        BehaviorState::Gather => "채집",
        BehaviorState::Trade => "거래",
        BehaviorState::Hunt => "사냥",
        BehaviorState::Rest => "회복",
    }
}

pub fn faction_label(faction: Faction) -> &'static str {
    match faction {
        Faction::Neutral => "중립 연합",
        Faction::MerchantGuild => "상인 길드",
        Faction::BanditClans => "산적 연맹",
        Faction::ExplorersLeague => "탐험가 연맹",
        Faction::SettlersUnion => "개척민 연합",
        Faction::TempleOfSuns => "태양의 성전",
    }
}

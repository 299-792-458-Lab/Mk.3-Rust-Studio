//! Structured metadata describing TERA's worldbuilding fabric.

use std::collections::HashMap;

use bevy_ecs::prelude::Resource;

use crate::simulation::{Biome, Faction, Position};

#[derive(Debug, Clone)]
pub struct BiomeMetadata {
    pub label: &'static str,
    pub epithet: &'static str,
    pub description: &'static str,
    pub anchor: (f32, f32),
    pub resource_profile: Vec<&'static str>,
    pub tensions: Vec<&'static str>,
}

#[derive(Debug, Clone)]
pub struct FactionMetadata {
    pub motto: &'static str,
    pub doctrine: &'static str,
    pub influence_vectors: Vec<&'static str>,
    pub strongholds: Vec<Biome>,
}

#[derive(Debug, Clone)]
pub struct EconomyMetadata {
    pub circulation_cycle: Vec<&'static str>,
    pub stressors: Vec<&'static str>,
    pub catalysts: Vec<&'static str>,
}

#[derive(Debug, Clone)]
pub struct EpochCadence {
    pub day_segments: Vec<&'static str>,
    pub seasons: Vec<&'static str>,
}

#[derive(Debug, Clone, Resource)]
pub struct WorldMetadata {
    pub biomes: HashMap<Biome, BiomeMetadata>,
    pub factions: HashMap<Faction, FactionMetadata>,
    pub economy: EconomyMetadata,
    pub epochs: EpochCadence,
}

impl WorldMetadata {
    pub fn anchor_position(&self, biome: Biome) -> Position {
        if let Some(metadata) = self.biomes.get(&biome) {
            Position {
                x: metadata.anchor.0,
                y: metadata.anchor.1,
                biome,
            }
        } else {
            Position {
                x: 0.0,
                y: 0.0,
                biome,
            }
        }
    }

    pub fn faction_profile(&self, faction: Faction) -> Option<&FactionMetadata> {
        self.factions.get(&faction)
    }

    pub fn epoch_for_tick(&self, tick: u64) -> (&'static str, &'static str) {
        let day_segments = &self.epochs.day_segments;
        let seasons = &self.epochs.seasons;

        let day_segment = day_segments[(tick as usize) % day_segments.len()];
        let season = seasons[((tick / day_segments.len() as u64) as usize) % seasons.len()];

        (day_segment, season)
    }
}

impl Default for WorldMetadata {
    fn default() -> Self {
        let biomes = [
            (
                Biome::Forest,
                BiomeMetadata {
                    label: "Verdant Veil",
                    epithet: "Whispers of the canopy",
                    description:
                        "Ancient woodland dense with medicinal herbs, hidden shrines, and feral spirits.",
                    anchor: (6.0, 4.5),
                    resource_profile: vec!["Herbs", "Timber", "Rare Fauna"],
                    tensions: vec!["Bandit ambushes", "Explorer expeditions", "Shrine guardians"],
                },
            ),
            (
                Biome::Plains,
                BiomeMetadata {
                    label: "Silverwind Expanse",
                    epithet: "Caravans under open skies",
                    description:
                        "Sweeping grasslands supporting caravans, crop rotations, and mounted patrols.",
                    anchor: (1.0, 2.0),
                    resource_profile: vec!["Grain", "Livestock", "Fiber"],
                    tensions: vec!["Harvest disputes", "Predator migrations", "Caravan tolls"],
                },
            ),
            (
                Biome::Desert,
                BiomeMetadata {
                    label: "Ashen Mirage",
                    epithet: "Ruins beneath shifting dunes",
                    description:
                        "Arid expanse studded with relic vaults and perilous mirages that test every expedition.",
                    anchor: (-4.0, -1.5),
                    resource_profile: vec!["Relics", "Minerals", "Glassroot"],
                    tensions: vec!["Water scarcity", "Sandstorms", "Relic skirmishes"],
                },
            ),
            (
                Biome::Village,
                BiomeMetadata {
                    label: "Hearthbound Circuit",
                    epithet: "Communal heartland",
                    description:
                        "Interlinked settlements with workshops, granaries, and Temple clinics.",
                    anchor: (3.5, -3.0),
                    resource_profile: vec!["Processed goods", "Craftsmanship", "Faith services"],
                    tensions: vec!["Civic disputes", "Disease outbreaks", "Supply shortfalls"],
                },
            ),
            (
                Biome::Market,
                BiomeMetadata {
                    label: "Golden Conflux",
                    epithet: "Pulse of commerce",
                    description:
                        "Tiered bazaar city where guild councils broker trade, tariffs, and diplomatic truces.",
                    anchor: (0.0, 0.0),
                    resource_profile: vec!["Currency", "Contracts", "Information"],
                    tensions: vec!["Tariff wars", "Speculation crashes", "Guild intrigue"],
                },
            ),
        ]
        .into_iter()
        .collect();

        let factions = [
            (
                Faction::MerchantGuild,
                FactionMetadata {
                    motto: "Balance the ledgers, steady the realm.",
                    doctrine: "Trade diplomacy, caravan escorts, and price harmonization.",
                    influence_vectors: vec![
                        "Tariff control",
                        "Supply contracts",
                        "Credit issuance",
                    ],
                    strongholds: vec![Biome::Market, Biome::Plains],
                },
            ),
            (
                Faction::BanditClans,
                FactionMetadata {
                    motto: "Take what the world hoards.",
                    doctrine: "Asymmetric raids, fear tactics, and relic hoarding.",
                    influence_vectors: vec!["Ambush threat", "Shadow markets", "Smuggler networks"],
                    strongholds: vec![Biome::Forest, Biome::Desert],
                },
            ),
            (
                Faction::ExplorersLeague,
                FactionMetadata {
                    motto: "Chart the unknown, claim the unseen.",
                    doctrine: "Survey missions, anomaly mapping, and relic authentication.",
                    influence_vectors: vec![
                        "Discovery rights",
                        "Cartographic data",
                        "Artifact appraisal",
                    ],
                    strongholds: vec![Biome::Forest, Biome::Desert],
                },
            ),
            (
                Faction::SettlersUnion,
                FactionMetadata {
                    motto: "Rooted in toil, risen by craft.",
                    doctrine: "Cooperative labor, agrarian planning, and civic rebuilding.",
                    influence_vectors: vec![
                        "Infrastructure projects",
                        "Harvest yields",
                        "Community festivals",
                    ],
                    strongholds: vec![Biome::Plains, Biome::Village],
                },
            ),
            (
                Faction::TempleOfSuns,
                FactionMetadata {
                    motto: "Three suns, one light of accord.",
                    doctrine: "Peace mediation, relic sanctification, and public welfare.",
                    influence_vectors: vec![
                        "Healing rites",
                        "Pilgrimage networks",
                        "Moral authority",
                    ],
                    strongholds: vec![Biome::Village, Biome::Market],
                },
            ),
        ]
        .into_iter()
        .collect();

        let economy = EconomyMetadata {
            circulation_cycle: vec![
                "Market auctions",
                "Merchant caravans",
                "Village services",
                "Desert expeditions",
                "Market remittance",
            ],
            stressors: vec![
                "Drought pressure",
                "Bandit surges",
                "Currency debasement",
                "Relic scarcity",
            ],
            catalysts: vec![
                "Temple festivals",
                "Explorer breakthroughs",
                "Guild tariff cuts",
                "Union harvest boons",
            ],
        };

        let epochs = EpochCadence {
            day_segments: vec!["Daybreak", "Highsun", "Nightfall"],
            seasons: vec!["Bloomrise", "Highflame", "Emberfall"],
        };

        Self {
            biomes,
            factions,
            economy,
            epochs,
        }
    }
}

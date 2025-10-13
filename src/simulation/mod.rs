use bevy_ecs::prelude::*;
use bevy_ecs::schedule::Schedule;

pub mod components;
pub mod resources;
pub mod systems;
pub mod world;

pub use components::*;
pub use resources::*;
pub use systems::*;
pub use world::*;

pub struct SimulationWorld {
    world: World,
    schedule: Schedule,
}

impl SimulationWorld {
    pub fn new(config: SimulationConfig) -> Self {
        let mut world = World::default();
        world.insert_resource(config);
        world.insert_resource(WorldTime::default());
        world.insert_resource(WorldMetadata::default());

        seed_entities(&mut world);

        let mut schedule = Schedule::default();
        schedule.add_systems(
            (
                ai_state_transition_system,
                movement_and_combat_system,
                economy_system,
                logging_system,
            )
                .chain(),
        );

        Self { world, schedule }
    }

    pub fn tick(&mut self) {
        {
            let mut time = self.world.resource_mut::<WorldTime>();
            time.tick += 1;
        }

        self.schedule.run(&mut self.world);
    }

    pub fn tick_count(&self) -> u64 {
        self.world.resource::<WorldTime>().tick
    }
}

fn seed_entities(world: &mut World) {
    use BehaviorState::*;

    let world_meta = world.resource::<WorldMetadata>().clone();

    let npc_templates = [
        (
            Identity {
                id: 1,
                name: "Calix".to_string(),
                faction: Faction::MerchantGuild,
            },
            world_meta.anchor_position(Biome::Market),
            Inventory {
                items: vec![ItemStack {
                    item: ItemKind::Resource("Herbs".into()),
                    quantity: 10,
                }],
                currency: 100.0,
            },
            Attributes {
                health: 100.0,
                stamina: 80.0,
                wealth: 120.0,
                fame: 20.0,
            },
            Personality {
                aggressive: 0.1,
                cautious: 0.4,
                social: 0.6,
                curious: 0.5,
            },
            Behavior { state: Idle },
        ),
        (
            Identity {
                id: 2,
                name: "Rena".to_string(),
                faction: Faction::BanditClans,
            },
            world_meta.anchor_position(Biome::Forest),
            Inventory {
                items: vec![ItemStack {
                    item: ItemKind::Equipment("Dagger".into()),
                    quantity: 1,
                }],
                currency: 45.0,
            },
            Attributes {
                health: 110.0,
                stamina: 95.0,
                wealth: 60.0,
                fame: 45.0,
            },
            Personality {
                aggressive: 0.6,
                cautious: 0.2,
                social: 0.3,
                curious: 0.4,
            },
            Behavior { state: Explore },
        ),
        (
            Identity {
                id: 3,
                name: "Aria".to_string(),
                faction: Faction::ExplorersLeague,
            },
            world_meta.anchor_position(Biome::Plains),
            Inventory {
                items: vec![],
                currency: 70.0,
            },
            Attributes {
                health: 95.0,
                stamina: 100.0,
                wealth: 80.0,
                fame: 35.0,
            },
            Personality {
                aggressive: 0.2,
                cautious: 0.3,
                social: 0.5,
                curious: 0.7,
            },
            Behavior { state: Gather },
        ),
        (
            Identity {
                id: 4,
                name: "Lys".to_string(),
                faction: Faction::TempleOfSuns,
            },
            world_meta.anchor_position(Biome::Village),
            Inventory {
                items: vec![ItemStack {
                    item: ItemKind::Artifact("Sun Reliquary".into()),
                    quantity: 1,
                }],
                currency: 30.0,
            },
            Attributes {
                health: 90.0,
                stamina: 70.0,
                wealth: 50.0,
                fame: 65.0,
            },
            Personality {
                aggressive: 0.1,
                cautious: 0.5,
                social: 0.7,
                curious: 0.6,
            },
            Behavior { state: Idle },
        ),
    ];

    for (identity, position, inventory, attributes, personality, behavior) in npc_templates {
        world.spawn((
            identity,
            position,
            inventory,
            attributes,
            personality,
            behavior,
        ));
    }
}

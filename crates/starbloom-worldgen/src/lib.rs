use bevy_ecs::prelude::*;
use noise::Simplex;

use starbloom_base::*;

mod biome;

use crate::biome::*;

pub struct WorldgenPlugin();

impl Plugin for WorldgenPlugin {
    fn create(world: &mut World, _schedule: &mut Schedule) {
        world.insert_resource(WorldgenProvider::new(0));
    }
}

#[derive(Resource)]
pub struct WorldgenProvider {
    biome_noise: Simplex,
    biomes: std::collections::HashMap<&'static str, Biome>,
}

impl WorldgenProvider {
    fn new(seed: u64) -> Self {
        Self {
            biome_noise: Simplex::new(0),
            biomes: std::collections::HashMap::new(),
        }
    }
}

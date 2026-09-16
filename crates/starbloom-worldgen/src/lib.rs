use noise::Simplex;

use starbloom_base::prelude::*;
use starbloom_map::*;

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
    fn new(seed: u32) -> Self {
        Self {
            biome_noise: Simplex::new(seed),
            biomes: std::collections::HashMap::new(),
        }
    }

    pub fn generate_chunk_data(chunk_pos: Vec2) -> [[u16; CHUNK_DIM]; CHUNK_DIM] {
        [[1; CHUNK_DIM]; CHUNK_DIM]
    }
}

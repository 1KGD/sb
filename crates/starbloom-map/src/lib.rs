use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use starbloom_base::*;

const CHUNK_DIM: usize = 16;
const TILE_SIZE: f32 = 16.;

#[repr(u16)]
#[derive(PartialEq)]
enum Tile {
    AIR,
    DIRT,
    SAND,
}

struct Chunk {
    tiles: [[Tile; CHUNK_DIM]; CHUNK_DIM],
}

impl Chunk {
    pub fn render(&self) {
        for (x, row) in self.tiles.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                if *tile == Tile::AIR {
                    continue;
                }
                draw_rectangle(
                    x as f32 * TILE_SIZE,
                    y as f32 * TILE_SIZE,
                    TILE_SIZE,
                    TILE_SIZE,
                    GREEN,
                );
            }
        }
    }
}

#[derive(Resource, Default)]
pub struct TileMap {
    chunks: Vec<Vec<Chunk>>,
}

impl TileMap {
    pub fn new() -> Self {
        Self { chunks: vec![] }
    }
}

pub struct MapPlugin();

impl Plugin for MapPlugin {
    fn create(world: &mut World, schedule: &mut Schedule) {
        world.insert_resource(TileMap::new());
        schedule.add_systems(render_world_map);
    }
}

fn render_world_map(map: Res<TileMap>) {
    for row in &map.chunks {
        for chunk in row {
            chunk.render();
        }
    }
}

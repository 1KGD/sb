use std::collections::HashMap;

use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use starbloom_base::*;

mod tile;

pub use crate::tile::*;

const CHUNK_DIM: usize = 16;
const TILE_SIZE: f32 = 16.;

type TileRepr = u16;

#[derive(Component)]
struct Chunk {
    tiles: [[TileRepr; CHUNK_DIM]; CHUNK_DIM],
}

impl Chunk {
    pub fn render(&self, tile_regestry: &TileRegestry) {
        for (x, row) in self.tiles.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
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

    pub fn get(&self, x: usize, y: usize) -> TileRepr {
        self.tiles[x][y]
    }
}

pub trait TileType: Sync + Send {}

#[derive(Resource, Default)]
pub struct TileRegestry {
    map: HashMap<TileRepr, Box<dyn TileType>>,
}

pub struct MapPlugin();

impl Plugin for MapPlugin {
    fn create(world: &mut World, schedule: &mut Schedule) {
        world.insert_resource(TileRegestry::default());
        schedule.add_systems(render_chunks);
    }
}

fn render_chunks(query: Query<&Chunk>, tile_regestry: Res<TileRegestry>) {
    for chunk in &query {
        chunk.render(&tile_regestry);
    }
}

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

#[derive(Resource)]
pub struct TileRegestry {
    ids: std::collections::BTreeMap<TileRepr, &'static str>,
    entries: std::collections::HashMap<&'static str, &'static Tile>,
}

impl TileRegestry {
    pub fn new() -> Self {
        Self {
            ids: std::collections::BTreeMap::new(),
            entries: std::collections::HashMap::new(),
        }
    }

    pub fn regester(&mut self, id: &'static str, tile: &'static Tile) {
        self.entries.insert(id, tile);
        if self.ids.values().find(|v| **v == id).is_none() {
            self.ids.insert(self.ids.len() as TileRepr, id);
        }
    }
}

pub struct MapPlugin();

impl Plugin for MapPlugin {
    fn create(world: &mut World, schedule: &mut Schedule) {
        world.insert_resource(TileRegestry::new());
        schedule.add_systems(render_chunks);
    }
}

fn render_chunks(query: Query<&Chunk>, tile_regestry: Res<TileRegestry>) {
    for chunk in &query {
        chunk.render(&tile_regestry);
    }
}

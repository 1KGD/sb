use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use crate::tile::*;

const CHUNK_DIM: usize = 16;
const TILE_SIZE: f32 = 16.;

#[derive(Component)]
pub struct Chunk {
    tiles: [[TileRepr; CHUNK_DIM]; CHUNK_DIM],
}

impl Chunk {
    pub fn render(&self, tile_regestry: &TileRegestry) {
        for (x, row) in self.tiles.iter().enumerate() {
            for (y, tile_idx) in row.iter().enumerate() {
                let tile = tile_regestry.get_tile_by_idx(tile_idx);
                if !tile.renderable {
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

    pub fn get(&self, x: usize, y: usize) -> TileRepr {
        self.tiles[x][y]
    }
}

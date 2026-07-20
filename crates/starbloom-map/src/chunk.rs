use bevy_ecs::prelude::*;
use starbloom_tiles::*;

pub const CHUNK_DIM: usize = 16;

pub const CHUNK_SIZE: f32 = CHUNK_DIM as f32 * TILE_SIZE;

#[derive(Component)]
pub struct Chunk {
    x: u64, // No overflows for you any time soon
    y: u64,
    tiles: [[TileRepr; CHUNK_DIM]; CHUNK_DIM],
}

impl Chunk {
    pub fn load(x: u64, y: u64) -> Self {
        Self {
            x,
            y,
            tiles: [[1; CHUNK_DIM]; CHUNK_DIM],
        }
    }

    pub fn render(&self, tile_regestry: &TileRegestry) {
        for (x, row) in self.tiles.iter().enumerate() {
            for (y, tile_idx) in row.iter().enumerate() {
                let tile = tile_regestry.get_tile_by_idx(tile_idx);
                if !tile.renderable {
                    continue;
                }
            }
        }
    }

    pub fn get(&self, x: usize, y: usize) -> TileRepr {
        self.tiles[x][y]
    }
}

use bevy_ecs::prelude::*;
use macroquad::prelude::*;

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
                draw_rectangle(
                    self.x as f32 * CHUNK_SIZE + x as f32 * TILE_SIZE,
                    self.y as f32 * CHUNK_SIZE + y as f32 * TILE_SIZE,
                    TILE_SIZE,
                    TILE_SIZE,
                    GREEN,
                );
            }
        }

        let dim = measure_text(format!("{}, {}", self.x, self.y), None, 20, 1.);
        draw_text(
            format!("{}, {}", self.x, self.y),
            self.x as f32 * CHUNK_SIZE,
            self.y as f32 * CHUNK_SIZE + dim.height,
            20.,
            BLACK,
        );
    }

    pub fn get(&self, x: usize, y: usize) -> TileRepr {
        self.tiles[x][y]
    }

    pub fn get_bounding_rect(&self) -> Rect {
        Rect::new(
            self.x as f32 * CHUNK_SIZE,
            self.y as f32 * CHUNK_SIZE,
            CHUNK_SIZE,
            CHUNK_SIZE,
        )
    }
}

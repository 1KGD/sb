use starbloom_core_shared::prelude::*;
use starbloom_tiles::*;

pub const CHUNK_DIM: usize = 8;

#[sided(Client)]
pub const CHUNK_SIZE: f32 = CHUNK_DIM as f32 * TILE_SIZE;

#[derive(Component, Default)]
pub struct Chunk {
    pub(crate) pos: IVec2,
    pub(crate) tiles: Option<[[TileRepr; CHUNK_DIM]; CHUNK_DIM]>,
}

impl Chunk {
    pub(crate) fn get_world_pos(&self) -> Vec2 {
        self.pos.as_vec2() * CHUNK_SIZE
    }
}

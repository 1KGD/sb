use starbloom_core_shared::prelude::*;
use starbloom_tiles::*;

pub const CHUNK_DIM: usize = 8;

pub const CHUNK_SIZE: f32 = CHUNK_DIM as f32 * TILE_SIZE;

#[derive(Component, Default)]
pub struct Chunk {
    pub(crate) pos: IVec2,
    pub(crate) tiles: Option<[[TileRepr; CHUNK_DIM]; CHUNK_DIM]>,
}

impl Chunk {
    pub(crate) fn from_pos(pos: IVec2) -> Self {
        Self { pos, tiles: None }
    }

    pub(crate) fn get_world_pos(&self) -> Vec2 {
        self.pos.as_vec2() * CHUNK_SIZE
    }

    #[sided(Client)]
    pub(crate) fn get_bounding_rect(&self) -> Rect {
        Rect::new(self.get_world_pos(), Vec2::splat(CHUNK_SIZE))
    }

    #[sided(Client)]
    pub(crate) fn should_be_culled(&self, viewport: Rect) -> bool {
        for vert in self.get_bounding_rect().corners() {
            if viewport.contains(vert) {
                return false;
            }
        }
        true
    }
}

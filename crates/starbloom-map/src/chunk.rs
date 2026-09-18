use starbloom_base::prelude::*;
use starbloom_camera::*;
use starbloom_tiles::*;

use crate::data::*;

pub const CHUNK_DIM: usize = 8;

pub const CHUNK_SIZE: f32 = CHUNK_DIM as f32 * TILE_SIZE;

#[derive(Component)]
pub struct Chunk {
    x: u64, // No overflows for you any time soon
    y: u64,
    tiles: Option<[[TileRepr; CHUNK_DIM]; CHUNK_DIM]>,
}

impl Chunk {
    pub fn load(x: u64, y: u64) -> Self {
        Self { x, y, tiles: None }
    }

    pub fn render(
        &self,
        renderer: &mut NonSendMut<Renderer>,
        main_camera: &Res<MainCamera>,
        tile_regestry: &Res<TileRegestry>,
    ) {
        if let Some(tiles) = self.tiles {
            if let Some(mut ctx) = renderer.ctx() {
                for (x, row) in tiles.iter().enumerate() {
                    for (y, tile_idx) in row.iter().enumerate() {
                        let tile = tile_regestry.get_tile_by_idx(tile_idx);
                        let pos = main_camera.cam.world_to_screen(
                            vec2(x as f32, y as f32) * TILE_SIZE
                                + vec2(self.x as f32, self.y as f32) * CHUNK_SIZE,
                        );
                        ctx.gfx.rect().size(vec2(TILE_SIZE, TILE_SIZE)).at(pos);
                    }
                }
            }
        }
    }

    pub fn generate(&mut self, data_provider: &ChunkDataProvider) {
        self.tiles = Some(data_provider.get_chunk_data(self.x, self.y));
    }

    pub fn get(&self, x: usize, y: usize) -> Option<TileRepr> {
        self.tiles.map(|tiles: [[u16; 8]; 8]| tiles[x][y])
    }
}

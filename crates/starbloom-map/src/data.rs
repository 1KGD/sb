use starbloom_base::prelude::*;

use crate::chunk::*;

#[derive(Resource)]
pub struct ChunkDataProvider;

impl ChunkDataProvider {
    pub fn get_chunk_data(&self, pos: IVec2) -> [[u16; CHUNK_DIM]; CHUNK_DIM] {
        [[1; CHUNK_DIM]; CHUNK_DIM]
    }
}

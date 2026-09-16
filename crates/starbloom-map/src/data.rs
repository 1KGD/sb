use starbloom_base::prelude::*;

use crate::chunk::*;

#[derive(Resource)]
pub struct ChunkDataProvider;

impl ChunkDataProvider {
    pub fn get_chunk_data(&self, x: u64, y: u64) -> [[u16; CHUNK_DIM]; CHUNK_DIM] {
        [[1; CHUNK_DIM]; CHUNK_DIM]
    }
}

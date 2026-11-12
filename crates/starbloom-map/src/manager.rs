use std::collections::HashSet;

use crate::*;

#[derive(Resource)]
pub(crate) struct ChunkManager {
    cache: HashSet<IVec2>,
}

impl ChunkManager {
    pub fn new() -> Self {
        Self {
            cache: HashSet::new(),
        }
    }

    pub fn add_chunk(&mut self, pos: IVec2) {
        self.cache.insert(pos);
    }

    pub fn remove_chunk(&mut self, pos: IVec2) {
        self.cache.remove(&pos);
    }

    pub fn has_chunk(&self, pos: IVec2) -> bool {
        self.cache.contains(&pos)
    }
}

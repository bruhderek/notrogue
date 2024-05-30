use chunk::ChunkManager;

pub mod tile;
pub mod chunk;

pub struct World {
    chunk_manager: ChunkManager
}

impl World {
    pub fn new() -> Self {
        World { chunk_manager: ChunkManager::new() }
    }
}
use std::collections::HashMap;

use super::tile::{tile_by_id, Tile, TileId, TILE_EMPTY};

pub struct ChunkManager {
    chunks: HashMap<(i32, i32), Chunk>,
}

impl ChunkManager {
    pub fn new() -> Self {
        ChunkManager {
            chunks: HashMap::new(),
        }
    }

    pub fn get_chunk(&self, cx: i32, cy: i32) -> Option<&Chunk> {
        self.chunks.get(&(cx, cy))
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<&'static Tile> {
        match self.get_chunk(x.div_euclid(16), y.div_euclid(16)) {
            Some(chunk) => chunk.get_tile(x.rem_euclid(16) as u32, y.rem_euclid(16) as u32),
            None => None,
        }
    }
}

pub struct Chunk {
    tiles: [[TileId; 16]; 16],
}

impl Chunk {
    fn new() -> Self {
        Chunk {
            tiles: [[TILE_EMPTY; 16]; 16],
        }
    }

    fn get_tile_id(&self, x: u32, y: u32) -> TileId {
        self.tiles[x as usize][y as usize]
    }

    fn get_tile(&self, x: u32, y: u32) -> Option<&'static Tile> {
        tile_by_id(self.get_tile_id(x, y))
    }
}

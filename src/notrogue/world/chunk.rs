use std::collections::HashMap;

use super::tile::{tile_by_id, Tile, TileId};

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

    pub fn get_chunk_mut(&mut self, cx: i32, cy: i32) -> Option<&mut Chunk> {
        self.chunks.get_mut(&(cx, cy))
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<&'static Tile> {
        match self.get_chunk(x.div_euclid(16), y.div_euclid(16)) {
            Some(chunk) => chunk.get_tile(x.rem_euclid(16) as u32, y.rem_euclid(16) as u32),
            None => None,
        }
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile: TileId) {
        match self.get_chunk_mut(x.div_euclid(16), y.div_euclid(16)) {
            Some(chunk) => {
                chunk.set_tile(x.rem_euclid(16) as u32, y.rem_euclid(16) as u32, tile);
            },
            None => {
                let mut c = Chunk::new();
                c.set_tile(x.rem_euclid(16) as u32, y.rem_euclid(16) as u32, tile);
                self.chunks.insert((x.div_euclid(16), y.div_euclid(16)), c);
            },
        };
    }
}

pub struct Chunk {
    tiles: [[TileId; 16]; 16],
}

impl Chunk {
    fn new() -> Self {
        Chunk {
            tiles: [[TileId::EMPTY; 16]; 16],
        }
    }

    fn get_tile_id(&self, x: u32, y: u32) -> TileId {
        self.tiles[x as usize][y as usize]
    }

    fn get_tile(&self, x: u32, y: u32) -> Option<&'static Tile> {
        tile_by_id(self.get_tile_id(x, y))
    }

    fn set_tile(&mut self, x: u32, y: u32, tile: TileId) {
        self.tiles[x as usize][y as usize] = tile;
    }
}

use chunk::{Chunk, ChunkManager};
use entity::{EntityData, EntityType};
use tile::{Tile, TileId};

pub mod chunk;
pub mod entity;
pub mod tile;

pub struct World {
    pub ecs_world: legion::World,
    pub player_entity: Option<legion::Entity>,
    chunk_manager: ChunkManager,
}

impl World {
    pub fn new() -> Self {
        World {
            ecs_world: legion::World::default(),
            player_entity: None,
            chunk_manager: ChunkManager::new(),
        }
    }

    pub fn initialize(&mut self) {
        self.player_entity = Some(self.new_entity(EntityType::PLAYER, (0, 0)));
    }

    pub fn new_entity(&mut self, entity_type: EntityType, location: (i32, i32)) -> legion::Entity {
        self.ecs_world
            .push((EntityData::new(entity_type, location.0, location.1),))
    }

    pub fn get_chunk(&self, cx: i32, cy: i32) -> Option<&Chunk> {
        self.chunk_manager.get_chunk(cx, cy)
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<&'static Tile> {
        self.chunk_manager.get_tile(x, y)
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile: TileId) {
        self.chunk_manager.set_tile(x, y, tile);
    }

    pub fn get_player(&mut self) -> EntityData {
        *self.ecs_world.entry(self.player_entity.unwrap()).unwrap().get_component::<EntityData>().unwrap()
    }
}

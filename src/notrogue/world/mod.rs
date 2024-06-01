use chunk::ChunkManager;
use entity::{EntityData, EntityType};

pub mod chunk;
pub mod entity;
pub mod tile;

pub struct World {
    ecs_world: legion::World,
    player_entity: Option<legion::Entity>,
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
}

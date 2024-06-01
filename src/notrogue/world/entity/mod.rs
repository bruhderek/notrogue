pub enum EntityType {
    PLAYER,
    DUMMY,
}

pub struct EntityData {
    entity_type: EntityType,
    pos_x: i32,
    pos_y: i32,
}

impl EntityData {
    pub fn new(entity_type: EntityType, pos_x: i32, pos_y: i32) -> Self {
        EntityData {
            entity_type,
            pos_x,
            pos_y,
        }
    }
}

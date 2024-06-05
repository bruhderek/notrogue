static TILES_BY_ID: &[Tile] = &[Tile::new("arch", Some("arch"), None)];

#[repr(usize)]
#[derive(PartialEq, Clone, Copy)]
pub enum TileId {
    ARCH = 0,
    EMPTY = usize::MAX,
}

pub fn tile_by_id(id: TileId) -> Option<&'static Tile> {
    if id != TileId::EMPTY {
        if (id as usize) < TILES_BY_ID.len() {
            Some(&TILES_BY_ID[id as usize])
        } else {
            None
        }
    } else {
        None
    }
}

pub struct Tile {
    pub id: &'static str,
    pub texture: Option<&'static str>,
    pub methods: Option<Box<dyn TileTrait>>,
}

impl Tile {
    pub const fn new(
        id: &'static str,
        texture: Option<&'static str>,
        methods: Option<Box<dyn TileTrait>>,
    ) -> Self {
        Tile {
            id,
            texture,
            methods,
        }
    }
}

pub trait TileTrait: Sync {
    fn get_texture(&self, x: i32, y: i32) -> &'static str;
}

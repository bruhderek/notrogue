pub static TILE_EMPTY: TileId = usize::MAX;

static TILES_BY_ID: &[Tile] = &[Tile::new("arch", Some("arch"), None)];

pub fn tile_by_id(id: TileId) -> Option<&'static Tile> {
    if id != TILE_EMPTY {
        if id < TILES_BY_ID.len() {
            Some(&TILES_BY_ID[id])
        } else {
            None
        }
    } else {
        None
    }
}

pub type TileId = usize;
pub struct Tile {
    id: &'static str,
    texture: Option<&'static str>,
    methods: Option<Box<dyn TileTrait>>,
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

use notcurses::{Notcurses, NotcursesResult, Plane};

use crate::{notrogue::{world::{tile::Tile, World}}, resource::get_resource};

pub fn get_center_pos(cli: &mut Plane) -> (u32, u32) {
    ((cli.size().w() as u32 / 2-1) / 2, (cli.size().h() as u32-1) / 2)
}

pub fn get_world_pos(world: &mut World, cli: &mut Plane, pos: (u32, u32)) -> (i32, i32) {
    (pos.0 as i32 - get_center_pos(cli).0 as i32 - world.get_player().pos_x, 
     pos.1 as i32 - get_center_pos(cli).1 as i32 - world.get_player().pos_y)
}

pub fn render_block(nc: &mut Notcurses, cli: &mut Plane, x: u32, y: u32, block: &Tile) -> NotcursesResult<Plane> {
    let mut p = cli.new_child_sized_at((2, 1), (x*2, y))?;
    get_resource(block.texture.expect("no block texture").to_string()).borrow_mut().blit_plane(nc, &mut p)?;
    Ok(p)
}
use util::{get_world_pos, render_block};

use super::{world::World};

mod util;

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Renderer {}
    }

    pub fn on_render(
        &self,
        world: &mut World,
        nc: &mut notcurses::Notcurses,
        cli: &mut notcurses::Plane,
    ) -> notcurses::NotcursesResult<()> {
        let mut planes = Vec::new();
        let mut counter: u32 = 0;
        for x in 0..cli.size().w() as u32 / 2 {
            for y in 0..cli.size().h() as u32 {
                let pos = get_world_pos(world, cli, (x, y));
                if let Some(n) = world.get_tile(pos.0, pos.1) {
                    counter += 1;
                    if let Ok(p) = render_block(nc, cli, x, y, n) {
                        planes.push(p);
                    }
                }
            }
        }
        let xy = get_world_pos(world, cli, (27, 57));
        cli.putstr_at_xy(Some(0), Some(0), (xy.0.to_string() + " " + &xy.1.to_string()).as_str())?;
        cli.putstr_at_xy(Some(0), Some(1), (cli.size().w().to_string() + " " + &cli.size().h().to_string()).as_str())?;
        cli.render()?;
        Ok(())
    }
}

use std::ops::Add;

use notcurses::{Channel, NotcursesError, Plane};
use util::{get_center_pos, get_screen_pos, get_world_pos, is_in_bounds, render_block, render_resource};

use crate::resource::{get_resource, static_resource};

use super::world::{player::controller::{Direction, PlayerController}, World};

mod util;

pub struct Renderer {}

impl Renderer {
    fn render_blocks(&self, world: &mut World, nc: &mut notcurses::Notcurses, cli: &mut notcurses::Plane, planes: &mut Vec<Plane>) -> notcurses::NotcursesResult<()> {
        for x in 0..cli.size().w() as u32 / 2 {
            for y in 0..cli.size().h() as u32 {
                let pos = get_world_pos(world, cli, (x, y));
                if let Some(n) = world.get_tile(pos.0, pos.1) {
                    let e_block = render_block(nc, cli, x, y, n);
                    if let Ok(block) = e_block {
                        planes.push(block);
                    } else {
                        return Err(NotcursesError::Message(String::from("render failed")));
                    }
                }
            }
        }
        Ok(())
    }

    fn render_debug(&self, world: &mut World, nc: &mut notcurses::Notcurses, cli: &mut notcurses::Plane, planes: &mut Vec<Plane>) -> notcurses::NotcursesResult<()> {
        let center_pos = get_center_pos(cli);
        cli.putstr_at_xy(Some(0), Some(0), (center_pos.0.to_string() + " " + &center_pos.1.to_string()).as_str())?;
        cli.putstr_at_xy(Some(0), Some(1), (cli.size().w().to_string() + " " + &cli.size().h().to_string()).as_str())?;
        Ok(())
    }

    fn render_player_movement(&self, world: &mut World, controller: &mut PlayerController, nc: &mut notcurses::Notcurses, cli: &mut notcurses::Plane, planes: &mut Vec<Plane>) -> notcurses::NotcursesResult<()> {
        let mut current_pos = (world.get_player().pos_x, world.get_player().pos_y);
        for a in &controller.steps {
            match *a {
                Direction::K => current_pos = (current_pos.0, current_pos.1-1),
                Direction::J => current_pos = (current_pos.0, current_pos.1+1),
                Direction::H => current_pos = (current_pos.0-1, current_pos.1),
                Direction::L => current_pos = (current_pos.0+1, current_pos.1),
            }
            let screen_pos = get_screen_pos(world, cli, current_pos);
            if is_in_bounds(cli, screen_pos) {
                let mut p = cli.new_child_sized_at((2, 1), (screen_pos.0*2, screen_pos.1))?;
                let mut channel = Channel::from_rgb((100, 100, 140));
                channel.set_alpha(notcurses::Alpha::Blend);
                p.set_bg(channel);
                p.putstr_at_xy(Some(0), Some(0), "  ")?;
                planes.push(p);
            }
        }
        Ok(())
    }
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {}
    }

    pub fn on_render(
        &self,
        world: &mut World,
        controller: &mut PlayerController,
        nc: &mut notcurses::Notcurses,
        cli: &mut notcurses::Plane,
    ) -> notcurses::NotcursesResult<()> {
        let mut planes = Vec::new();

        self.render_blocks(world, nc, cli, &mut planes)?;
        self.render_player_movement(world, controller, nc, cli, &mut planes)?;

        let center = get_center_pos(cli);
        let mut p = cli.new_child_sized_at((2, 1), (center.0*2, center.1))?;
        get_resource("knight".to_string()).borrow_mut().blit_plane(nc, &mut p)?;

        self.render_debug(world, nc, cli, &mut planes)?;

        cli.render()?;
        Ok(())
    }
}

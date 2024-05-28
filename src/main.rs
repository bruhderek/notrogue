use game::start_game_loop;
use notcurses::*;

pub mod resource;
mod game;
mod screen;

fn main() -> NotcursesResult<()> {
    let mut nc = Notcurses::new()?;
    let mut cli = nc.cli_plane()?;

    start_game_loop(&mut nc, &mut cli)?;

    Ok(())
}

use game::start_game_loop;
use notcurses::*;

mod game;
pub mod notrogue;
pub mod resource;
mod screen;

fn main() -> NotcursesResult<()> {
    let mut nc = Notcurses::new()?;
    let mut cli = nc.cli_plane()?;

    start_game_loop(&mut nc, &mut cli)?;

    Ok(())
}

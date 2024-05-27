use notcurses::{Notcurses, NotcursesResult, Plane};

pub fn fill_plane(_nc: &mut Notcurses, cli: &mut Plane, width: u32, height: u32) -> NotcursesResult<()> {
    for i in 0..height {
        cli.putstr_at_xy(Some(0), Some(i), &" ".repeat(width as usize))?;
    }
    Ok(())
}

pub fn get_mouse_xy(event: &notcurses::Input) -> Option<(u32, u32)> {
    if let Some(pos) = event.cell {
        return Some((pos.x() as u32, pos.y() as u32))
    }
    None
}
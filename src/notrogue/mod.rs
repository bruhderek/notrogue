use notcurses::Plane;
use util::get_center_xy;

use crate::{resource::get_resource, screen::{button::ButtonContainer, ScreenTrait}};

mod util;
mod world;

pub struct NotRogue {
    buttons: ButtonContainer,
}

impl NotRogue {
    pub fn new() -> Self {
        NotRogue {
            buttons: ButtonContainer { buttons: Vec::new() }
        }
    }
}

impl ScreenTrait for NotRogue {
    fn on_render(&self, nc: &mut notcurses::Notcurses, cli: &mut notcurses::Plane) -> notcurses::NotcursesResult<()> {
        let mut player = cli.new_child_sized_at((2, 1), get_center_xy(cli))?;
        get_resource("arch".to_string()).borrow_mut().blit_plane(nc, &mut player)?;
        cli.render()?;
        Ok(())
    }

    fn on_press_key(
            &self,
            _event: &notcurses::Input,
            _nc: &mut notcurses::Notcurses,
            _cli: &mut notcurses::Plane,
        ) -> notcurses::NotcursesResult<()> {
        Ok(())
    }
}
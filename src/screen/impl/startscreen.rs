use notcurses::{Key, NotcursesResult};

use crate::screen::{button::ButtonContainer, ScreenTrait};

pub struct StartScreen {
    pub buttons: ButtonContainer
}

impl ScreenTrait for StartScreen {
    fn on_render(&self, nc: &mut notcurses::Notcurses, cli: &mut notcurses::Plane) -> NotcursesResult<()> {
        cli.putstrln("sadflkasjdflkj")?;
        Ok(())
    }

    fn on_press_key(&self, event: &notcurses::Input, nc: &mut notcurses::Notcurses, cli: &mut notcurses::Plane) -> NotcursesResult<()> {
        if event.is_key(Key::Button1) {
            
        }
        Ok(())
    }
}
use std::{cell::RefCell};

use notcurses::{NotcursesError, NotcursesResult};

use crate::screen::{button::ButtonContainer, ScreenTrait};

use super::NotRogue;

thread_local! {
    pub static GAME: RefCell<Option<NotRogue>> = const { RefCell::new(None) };
}

pub struct NotRogueScreen {
    buttons: ButtonContainer,
}

impl Default for NotRogueScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl NotRogueScreen {
    pub fn new() -> Self {
        NotRogueScreen {
            buttons: ButtonContainer {
                buttons: Vec::new(),
            },
        }
    }
}

impl ScreenTrait for NotRogueScreen {
    fn on_create(&self) {
        GAME.set(Some(NotRogue::new()));
    }

    fn reset_data(&self) {
        GAME.set(None);
    }

    fn on_render(
        &self,
        nc: &mut notcurses::Notcurses,
        cli: &mut notcurses::Plane,
    ) -> notcurses::NotcursesResult<()> {
        GAME.with_borrow_mut(|game| -> NotcursesResult<()> {
            if let Some(game) = game {
                game.on_render(nc, cli)?;
                Ok(())
            } else {
                Err(NotcursesError::Message(String::from("game is not initialized")))
            }
        })?;
        Ok(())
    }

    fn on_update(&self) -> NotcursesResult<()> {
        GAME.with_borrow_mut(|game| -> NotcursesResult<()> {
            if let Some(game) = game {
                game.on_update()?;
                Ok(())
            } else {
                Err(NotcursesError::Message(String::from("game is not initialized")))
            }
        })?;
        Ok(())
    }

    fn on_press_key(
        &self,
        event: &notcurses::Input,
        nc: &mut notcurses::Notcurses,
        cli: &mut notcurses::Plane,
    ) -> notcurses::NotcursesResult<()> {
        GAME.with_borrow_mut(|game| {
            if let Some(game) = game {
                game.on_press_key(event, nc, cli)?;
                Ok(())
            } else {
                Err(NotcursesError::Message(String::from("game is not initialized",)))
            }
        })?;
        Ok(())
    }
}

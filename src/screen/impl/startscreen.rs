use std::{cell::RefCell, ops::Add};

use notcurses::{Key, NotcursesResult};

use crate::{game::CURRENT_SCREEN, screen::{
    button::{Button, ButtonContainer},
    util::get_mouse_xy,
    ScreenTrait,
}};

thread_local! {
    pub static COUNTER: RefCell<u32> = const { RefCell::new(0) };
    pub static EVENT: RefCell<String> = RefCell::new("".to_string());
}

pub struct StartScreen {
    buttons: ButtonContainer,
}

impl StartScreen {
    pub fn new() -> Self {
        StartScreen {
            buttons: ButtonContainer {
                buttons: vec![
                    Button::new(2, 3, 20, 7, "PLAY".to_string()),
                ],
            },
        }
    }
}

impl ScreenTrait for StartScreen {
    fn on_render(
        &self,
        nc: &mut notcurses::Notcurses,
        cli: &mut notcurses::Plane,
    ) -> NotcursesResult<()> {
        cli.putstrln(&COUNTER.with_borrow(|v| *v).to_string())?;
        cli.putstr_at_xy(Some(0), Some(1), &EVENT.with_borrow(|v| (*v).clone()))?;
        let _p = self.buttons.render_all(nc, cli)?;
        cli.putstr_at_xy(Some(1), Some(5), "helo")?;
        cli.render()?;
        Ok(())
    }

    fn on_press_key(
        &self,
        event: &notcurses::Input,
        _nc: &mut notcurses::Notcurses,
        _cli: &mut notcurses::Plane,
    ) -> NotcursesResult<()> {
        EVENT.with_borrow_mut(|v| {
            *v = event.to_string();
        });
        if event.is_key(Key::Button1) && event.is_press() {
            if let Some(xy) = get_mouse_xy(event) {
                if let Some(_i) = self.buttons.get_pressed_button(xy.0, xy.1) {
                    COUNTER.with_borrow_mut(|v| {
                        *v = v.add(1);
                    });
                    CURRENT_SCREEN.with_borrow_mut(|v| {
                        *v = 1;
                    });
                }
            }
        }
        Ok(())
    }
}

use std::{borrow::{Borrow, BorrowMut}, cell::{Cell, RefCell}};

use lazy_static::lazy_static;
use notcurses::{Key, MiceEvents, Notcurses, NotcursesResult, Plane};

use crate::{notrogue::screen::NotRogueScreen, resource::add_resources, screen::{self, r#impl::startscreen::StartScreen, Screen, ScreenTrait}
};

lazy_static! {
    pub static ref SCREENS: &'static [Screen] = Box::leak(Box::new([
        Screen::new(10, 10, Box::new(StartScreen::new())),
        Screen::new(10, 10, Box::new(NotRogueScreen::new()))
    ]));
}

pub fn current_screen() -> &'static Screen {
    &SCREENS[CURRENT_SCREEN.get()]
}

pub fn set_screen(x: usize) {
    CURRENT_SCREEN.set(x);
    current_screen().methods.on_create();
}

thread_local! {
    pub static CURRENT_SCREEN: Cell<usize> = const { Cell::new(0) };
}

pub fn initialize_game(nc: &mut Notcurses, cli: &mut Plane) -> NotcursesResult<()> {
    cli.erase();
    add_resources();
    nc.mice_enable(MiceEvents::Button)?;

    Ok(())
}

pub fn start_game_loop(nc: &mut Notcurses, cli: &mut Plane) -> NotcursesResult<()> {
    initialize_game(nc, cli)?;

    loop {
        let event = nc.poll_event()?;
        if event.is_press() {
            SCREENS[CURRENT_SCREEN.get()]
                .methods
                .on_press_key(&event, nc, cli)
                .expect("press event failed");
        }
        if event.is_key(Key::End) && event.is_press() {
            break;
        }
        cli.erase();
        SCREENS[CURRENT_SCREEN.get()]
            .methods
            .on_render(nc, cli)
            .expect("render failed");
    }
    Ok(())
}

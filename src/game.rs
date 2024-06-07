use std::{
    cell::{Cell},
};

use lazy_static::lazy_static;
use notcurses::{Key, MiceEvents, Notcurses, NotcursesResult, Plane};

use crate::{
    notrogue::screen::NotRogueScreen, resource::resources::add_resources, screen::{r#impl::startscreen::StartScreen, util::fill_plane, Screen, ScreenTrait}
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
    cli.set_bg((28, 28, 40));

    Ok(())
}

pub fn start_game_loop(nc: &mut Notcurses, cli: &mut Plane) -> NotcursesResult<()> {
    initialize_game(nc, cli)?;

    'main: loop {
        'input: loop {
            let event = nc.poll_event()?;
            if !event.received() {
                break 'input;
            }
            current_screen()
                .methods
                .on_press_key(&event, nc, cli)?;
            if event.is_key(Key::End) && event.is_press() {
                break 'main;
            }
        }
        current_screen().methods.on_update()?;
        cli.erase();
        fill_plane(nc, cli, cli.size().w() as u32, cli.size().h() as u32)?;
        current_screen()
            .methods
            .on_render(nc, cli)?;
    }
    Ok(())
}

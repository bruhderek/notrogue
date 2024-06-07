use notcurses::{Input, Notcurses, NotcursesResult, Plane};

pub struct Screen {
    pub width: u32,
    pub height: u32,
    pub methods: Box<dyn ScreenTrait>,
}

impl Screen {
    pub const fn new(width: u32, height: u32, methods: Box<dyn ScreenTrait>) -> Self {
        Screen {
            width,
            height,
            methods,
        }
    }
}

pub trait ScreenTrait: Sync {
    fn on_create(&self);
    fn reset_data(&self);
    fn on_render(&self, _nc: &mut Notcurses, _cli: &mut Plane) -> NotcursesResult<()> {
        Ok(())
    }
    fn on_update(&self) -> NotcursesResult<()> {
        Ok(())
    }
    fn on_press_key(
        &self,
        _event: &Input,
        _nc: &mut Notcurses,
        _cli: &mut Plane,
    ) -> NotcursesResult<()> {
        Ok(())
    }
}

pub mod button;
pub mod r#impl;
pub mod util;

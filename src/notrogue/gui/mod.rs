use notcurses::{Alpha, Channel, Notcurses, NotcursesResult, Plane};

use crate::screen::util::fill_plane;

pub mod logger;
mod util;

pub struct GuiContainer {
    pub guis: Vec<Gui>
}

impl GuiContainer {
    pub fn render_all(&self, nc: &mut Notcurses, cli: &mut Plane) -> NotcursesResult<Vec<Plane>> {
        let mut ret = Vec::new();
        for gui in self.guis.iter() {
            let mut new_plane = cli.new_child_sized_at((gui.w, gui.h), (gui.x, gui.y))?;
            new_plane.set_bg(Channel::from_rgb_alpha((0, 0, 0), Alpha::Blend));
            let size = new_plane.size();
            fill_plane(nc, &mut new_plane, size.w() as u32, size.h() as u32)?;
            let gui_planes = gui.methods.on_render(nc, &mut new_plane);
            if let Ok(mut planes) = gui_planes {
                ret.push(new_plane);
                ret.append(&mut planes);
            } else {
                return gui_planes
            }
        }
        Ok(ret)
    }

    pub fn update_all(&mut self) {
        for gui in self.guis.iter_mut() {
            gui.methods.on_update();
        }
    }
}

pub struct Gui {
    pub x: u32, pub y: u32,
    pub w: u32, pub h: u32,
    pub methods: Box<dyn GuiTrait>
}

impl Gui {
    pub fn new(x: u32, y: u32, w: u32, h: u32, methods: Box<dyn GuiTrait>) -> Self {
        Gui { x, y, w, h, methods }
    }
}

pub trait GuiTrait {
    fn on_render(&self, nc: &mut Notcurses, cli: &mut Plane) -> NotcursesResult<Vec<Plane>>;
    fn on_update(&mut self) {}
}
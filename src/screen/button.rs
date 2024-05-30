use notcurses::{Alpha, Channel, Notcurses, NotcursesResult, Plane};

use crate::resource::get_resource;

use super::util::fill_plane;

pub struct Button {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    text: String,
}

impl Button {
    pub fn new(x: u32, y: u32, width: u32, height: u32, text: String) -> Self {
        Button {
            x,
            y,
            width,
            height,
            text,
        }
    }

    fn is_pressed(&self, mouse_x: u32, mouse_y: u32) -> bool {
        (self.x <= mouse_x && mouse_x < self.x + self.width)
            && (self.y <= mouse_y && mouse_y < self.y + self.height)
    }

    fn render(&self, nc: &mut Notcurses, cli: &mut Plane) -> NotcursesResult<Plane> {
        let mut plane = cli.new_child_sized_at((self.width, self.height), (self.x, self.y))?;
        let mut channel = Channel::new();
        channel.set_rgb((50, 50, 50));
        channel.set_alpha(Alpha::Blend);
        plane.set_bg(channel);

        fill_plane(nc, &mut plane, self.width, self.height)?;
        plane.putstr_at_xy(
            Some((self.width - self.text.len() as u32) / 2),
            Some((self.height - 1) / 2),
            &self.text,
        )?;

        Ok(plane)
    }
}

pub struct ButtonContainer {
    pub buttons: Vec<Button>,
}

impl ButtonContainer {
    pub fn get_pressed_button(&self, mouse_x: u32, mouse_y: u32) -> Option<usize> {
        for (i, button) in self.buttons.iter().enumerate() {
            if button.is_pressed(mouse_x, mouse_y) {
                return Some(i);
            }
        }
        None
    }

    pub fn render_all(&self, nc: &mut Notcurses, cli: &mut Plane) -> NotcursesResult<Vec<Plane>> {
        let mut ret = Vec::new();
        for button in self.buttons.iter() {
            ret.push(button.render(nc, cli)?);
        }
        Ok(ret)
    }
}

use notcurses::{Notcurses, NotcursesResult, Plane};

pub struct Button {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    methods: Box<dyn ButtonTrait>,
}

impl Button {
    fn new(x: u32, y: u32, width: u32, height: u32, methods: Box<dyn ButtonTrait>) -> Self {
        Button {
            x,
            y,
            width,
            height,
            methods,
        }
    }

    fn is_pressed(&self, mouse_x: u32, mouse_y: u32) -> bool {
        (self.x <= mouse_x && mouse_x < self.x + self.width)
            && (self.y <= mouse_y && mouse_y < self.y + self.height)
    }

    fn render(&self, nc: &mut Notcurses, cli: &mut Plane) {
        let mut plane = cli.new_child_sized_at((self.width, self.height), (self.x, self.y));
        
    }
}

pub trait ButtonTrait {
    fn on_pressed(&self) -> NotcursesResult<()> {
        Ok(())
    }
}

pub struct ButtonContainer {
    pub buttons: Vec<Button>,
}

impl ButtonContainer {
    fn get_pressed_button(&self, mouse_x: u32, mouse_y: u32) -> Option<usize> {
        for (i, button) in self.buttons.iter().enumerate() {
            if button.is_pressed(mouse_x, mouse_y) {
                return Some(i);
            }
        }
        None
    }
}

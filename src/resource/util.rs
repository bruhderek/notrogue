use notcurses::{Visual, VisualOptions};
use notcurses::Blitter::Pixel;
use notcurses::Scale::Stretch;

pub fn configure_visual(v: &mut Visual) {
    let mut options = VisualOptions::default();
    options.set_scale(Stretch);
    options.set_blitter(Pixel);
    options.set_x(0);
    options.set_y(0);
    v.set_options(options);
}
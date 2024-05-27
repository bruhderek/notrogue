use notcurses::sys::c_api::ncinput;
use notcurses::Blitter::Pixel;
use notcurses::Scale::Stretch;
use notcurses::{Alpha, Channel, Notcurses, NotcursesResult, Plane, Visual, VisualOptions};

pub fn create_bob(nc: &mut Notcurses, cli: &mut Plane) -> NotcursesResult<()> {
    let mut background = Visual::builder().build_from_file("resources/bob.png")?;
    let mut options = VisualOptions::default();
    let mut plane = cli.new_child_sized_at((4, 2), (10, 10))?;
    options.set_scale(Stretch);
    options.set_blitter(Pixel);
    options.set_x(0);
    options.set_y(0);
    background.set_options(options);
    background.blit_plane(nc, &mut plane)?;
    cli.render()?;
    Ok(())
}

pub fn create_ui(_nc: &mut Notcurses, cli: &mut Plane, _input: &mut ncinput) -> NotcursesResult<()> {
    let mut plane = cli.new_child_sized_at((24, 10), (10, 5))?;
    plane.set_bg((255, 0, 0));
    for i in 0..10 {
        plane.putstr_at_xy(Some(0), Some(i), &" ".repeat(24))?;
    }
    let mut plane = cli.new_child_sized_at((6, 3), (20, 10))?;
    // let mut channel = NcChannels::new().bg_set_r(255).bg_set_g(255).bg_set_b(255);
    // channel.set_bg_alpha(NcAlpha::Blend)?;
    // let mut transparent = NcChannels::new();
    // transparent.set_bg_alpha(NcAlpha::Transparent)?;
    // let selector = NcSelectorBuilder::new().title("iuyghiuhy").
    //     all_channels(channel, channel, channel, channel, channel).finish(plane.into_ref_mut())?;
    // selector.additem(NcSelectorItem::new(&NcString::new("asdf"), &NcString::new("asdf")))?;
    // if input.ctrl {
    //     cli.putstr("hi")?;
    // }
    let mut channel = Channel::new();
    channel.set_rgb((255, 255, 255));
    channel.set_alpha(Alpha::Blend);
    plane.set_bg(channel);
    for i in 0..6 {
        plane.putstr_at_xy(Some(0), Some(i), &"a".repeat(6))?;
    }
    cli.render()?;
    Ok(())
}


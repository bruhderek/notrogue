pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Renderer {}
    }

    pub fn on_render(
        &self,
        _nc: &mut notcurses::Notcurses,
        cli: &mut notcurses::Plane,
    ) -> notcurses::NotcursesResult<()> {
        cli.render();
        Ok(())
    }
}

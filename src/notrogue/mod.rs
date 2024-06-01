use notcurses::Plane;
use renderer::Renderer;
use world::World;

pub mod screen;
mod util;
mod world;
mod renderer;

struct NotRogue {
    world: World,
    renderer: Renderer
}

impl NotRogue {
    pub fn new() -> Self {
        let mut world = World::new();
        world.initialize();
        NotRogue { world, renderer: Renderer::new() }
    }

    fn on_render(&self, nc: &mut notcurses::Notcurses, cli: &mut notcurses::Plane) -> notcurses::NotcursesResult<()> {
        // self.renderer.on_render(nc, cli);
        Ok(())
    }
}
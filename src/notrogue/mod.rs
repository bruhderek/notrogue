
use renderer::Renderer;
use world::World;

mod renderer;
pub mod screen;
mod util;
mod world;

struct NotRogue {
    world: World,
    renderer: Renderer,
}

impl NotRogue {
    pub fn new() -> Self {
        let mut world = World::new();
        world.initialize();
        NotRogue {
            world,
            renderer: Renderer::new(),
        }
    }

    fn on_render(
        &self,
        _nc: &mut notcurses::Notcurses,
        _cli: &mut notcurses::Plane,
    ) -> notcurses::NotcursesResult<()> {
        // self.renderer.on_render(nc, cli);
        Ok(())
    }
}

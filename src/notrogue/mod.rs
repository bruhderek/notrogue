
use renderer::Renderer;
use world::{tile::TileId, World};

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
        world.set_tile(0, 0, TileId::ARCH);
        world.set_tile(1, 0, TileId::ARCH);
        NotRogue {
            world,
            renderer: Renderer::new(),
        }
    }

    fn get_renderer(&mut self) -> &mut Renderer {
        &mut self.renderer
    }

    fn on_render(
        &mut self,
        nc: &mut notcurses::Notcurses,
        cli: &mut notcurses::Plane,
    ) -> notcurses::NotcursesResult<()> {
        self.renderer.on_render(&mut self.world, nc, cli)?;
        Ok(())
    }

    fn on_press_key(
        &self,
        _event: &notcurses::Input,
        _nc: &mut notcurses::Notcurses,
        _cli: &mut notcurses::Plane,
    ) -> notcurses::NotcursesResult<()> {
        Ok(())
    }
}

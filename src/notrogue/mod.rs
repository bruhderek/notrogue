
use notcurses::{Key};
use renderer::Renderer;
use world::{player::controller::PlayerController, tile::TileId, World};

mod renderer;
pub mod screen;
mod util;
mod world;

struct NotRogue {
    world: World,
    renderer: Renderer,
    controller: PlayerController
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
            controller: PlayerController::new()
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
        self.renderer.on_render(&mut self.world, &mut self.controller, nc, cli)?;
        Ok(())
    }

    fn on_update(&mut self) -> notcurses::NotcursesResult<()> {
        let messages = self.controller.on_update();
        for message in messages {
            self.world.process_player_message(message);
        }
        Ok(())
    }

    fn on_press_key(
        &mut self,
        event: &notcurses::Input,
        _nc: &mut notcurses::Notcurses,
        _cli: &mut notcurses::Plane,
    ) -> notcurses::NotcursesResult<()> {
        self.controller.on_press_key(event);
        Ok(())
    }
}

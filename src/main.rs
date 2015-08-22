#[macro_use] extern crate log;
extern crate env_logger;
extern crate sdl2;
extern crate time;

mod core {
    pub mod entity;
    pub mod sprite;
    pub mod square;
}

mod render {
    pub mod renderer;
    pub mod renderable;
}

use core::*;
use sdl2::Sdl;
use sdl2::keyboard::Keycode;

fn main() {
    env_logger::init().unwrap();
    let mut game = Game::new();
    game.run();
    game.close();
}

struct Game<'a> {
    render_system: render::renderer::RenderSystem<'a>,
    last_tick: u64,
    is_running: bool,
    sdl_context: Sdl,
    entities: Vec<Box<entity::Entity>>,
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        info!("Setting up game.");

        let sdl_context = sdl2::init().unwrap();
        let render_system = render::renderer::RenderSystem::new(&sdl_context);

        let mut entities: Vec<Box<entity::Entity>> = Vec::new();
        for _ in 0..5000 {
            entities.push(Box::new(square::Square::new(&render_system.sdl_renderer)));
        }

        Game {
            render_system: render_system,
            last_tick: time::precise_time_ns(),
            is_running: true,
            sdl_context: sdl_context,
            entities: entities,
        }
    }

    pub fn run(&mut self) {
        while self.is_running {
            self.check_events();
            self.tick();
            self.render();
        }
    }

    pub fn check_events(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;

            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    self.is_running = false
                },
                _ => {}
            }
        }
    }

    pub fn tick(&mut self) {
        let now = time::precise_time_ns();
        let ticks = now - self.last_tick;
        self.last_tick = now;

        for entity in self.entities.iter_mut() {
            entity.update(ticks);
        };
    }

    pub fn render(&mut self) {
        self.render_system.render(self.last_tick, &self.entities);
    }

    pub fn close(&self) {
        info!("Goodbye!");
    }
}

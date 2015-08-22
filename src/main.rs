#[macro_use] extern crate log;
extern crate env_logger;
extern crate sdl2;
extern crate time;

use sdl2::Sdl;
use sdl2::keyboard::Keycode;
use std::thread;

mod render {
    pub mod renderer;
}

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
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        info!("Setting up game.");

        let sdl_context = sdl2::init().unwrap();

        Game {
            render_system: render::renderer::RenderSystem::new(&sdl_context),
            last_tick: time::precise_time_ns(),
            is_running: true,
            sdl_context: sdl_context,
        }
    }

    pub fn run(&mut self) {
        while self.is_running {
            self.check_events();
            self.tick();
            self.render();

            thread::sleep_ms(33);
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
        self.last_tick = time::precise_time_ns();
    }

    pub fn render(&mut self) {
        self.render_system.render(self.last_tick);
    }

    pub fn close(&self) {
        info!("Goodbye!");
    }
}


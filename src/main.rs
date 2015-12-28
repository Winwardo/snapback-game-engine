#[macro_use]
extern crate log;
extern crate env_logger;
extern crate sdl2;
extern crate time;
extern crate rand;
#[macro_use]
extern crate bitflags;

extern crate nalgebra;

mod core;
mod render;

use core::*;
use sdl2::Sdl;
use sdl2::keyboard::Keycode;
use core::system::*;
use core::systems::physicssystem::*;
use core::entity::*;
use core::transforms::transform::*;
use core::physics::mass::*;
use core::physics::movement::*;
use core::world::*;
use core::times::tick::*;

fn main() {
    env_logger::init().unwrap();
    let mut game = Game::new();
    game.run();
    game.close();
}

struct Game {
    render_system: render::renderer::RenderSystem,
    last_tick: u64,
    is_running: bool,
    sdl_context: Sdl,
    world: World,
}

impl Game {
    pub fn new() -> Game {
        info!("Setting up game.");

        let sdl_context = sdl2::init().unwrap();

        let mut world = World {
            entities: Entities::new(),
            masses: Masses::new(),
            transforms: Transforms::new(),
            movements: Movements::new(),
        };

        // Clear compiler unused warning
        world.entities();
        world.masses();

        let mut render_system = render::renderer::RenderSystem::new(&sdl_context);

        info!("Creating entities");

        for _ in 0..1 {
            square::make_square(&mut world, &mut render_system);
        }

        let out = Game {
            render_system: render_system,
            last_tick: time::precise_time_ns(),
            is_running: true,
            sdl_context: sdl_context,
            world: world,
        };


        out
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
                }
                _ => {}
            }
        }
    }

    pub fn tick(&mut self) {
        let ticks = self.do_tick();

        process_physics(ticks, &mut self.world);
    }

    fn do_tick(&mut self) -> Ticks {
        let now = time::precise_time_ns();
        let ticks = now - self.last_tick;
        self.last_tick = now;

        let ticks_as_seconds = (ticks as f32 / 1000000000f32) as f32;
        ticks_as_seconds as Ticks
    }

    pub fn render(&mut self) {
        self.render_system.render(self.last_tick, &self.world);
    }

    pub fn close(&self) {
        info!("Goodbye!");
    }
}

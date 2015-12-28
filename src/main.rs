#[macro_use]
extern crate log;
extern crate env_logger;
extern crate sdl2;
extern crate time;
extern crate rand;
#[macro_use]
extern crate bitflags;

extern crate nalgebra;

mod core {
    pub mod entity;
    pub mod sprite;
    pub mod square;
    #[macro_use]    pub mod component;
    pub mod system;
    pub mod mass;
    pub mod systems {
        pub mod transformsystem;
        pub mod physicssystem;
    }
    pub mod transforms {
        pub mod position;
        pub mod rotation;
        pub mod transform;
    }
    pub mod times {
        pub mod tick;
    }
    pub mod world;
}

mod render {
    pub mod renderer;
    pub mod renderable;
}

use core::*;
use sdl2::Sdl;
use sdl2::keyboard::Keycode;
use core::system::*;
use core::systems::physicssystem::*;
use core::entity::*;
use core::transforms::position::*;
use core::transforms::rotation::*;
use core::transforms::transform::*;
use core::systems::transformsystem::*;
use core::component::*;
use core::mass::*;
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
    transform_system: TransformSystem,
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
        };

        let mut render_system = render::renderer::RenderSystem::new(&sdl_context);
        let mut transform_system = TransformSystem::new();

        info!("Creating entities");

        for _ in 0..1 {
            square::make_square(&mut world, &mut render_system, &mut transform_system);
        }

        let out = Game {
            render_system: render_system,
            transform_system: transform_system,
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
        self.render_system.render(self.last_tick, &self.transform_system, &self.world);
    }

    pub fn close(&self) {
        info!("Goodbye!");
    }
}

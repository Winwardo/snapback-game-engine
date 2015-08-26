#[macro_use] extern crate log;
extern crate env_logger;
extern crate sdl2;
extern crate time;
extern crate rand;
#[macro_use] extern crate bitflags;

extern crate nalgebra;

mod core {
    pub mod entity;
    pub mod sprite;
    pub mod square;
    pub mod transform;
    pub mod transformsystem;
    #[macro_use] pub mod component;
    pub mod system;
    pub mod physicssystem;
    pub mod mass;
    pub mod transforms {
        pub mod position;
    }
}

mod render {
    pub mod renderer;
    pub mod renderable;
}

use core::*;
use sdl2::Sdl;
use sdl2::keyboard::Keycode;
use std::rc::Rc;
use core::system::*;
use core::physicssystem::*;
use core::entity::*;
use core::transforms::position::*;
use core::component::*;
use core::mass::*;

fn main() {
    env_logger::init().unwrap();
    let mut game = Game::new();
    game.run();
    game.close();
}

struct Game {
    render_system: render::renderer::RenderSystem,
    transform_system: core::transformsystem::TransformSystem,
    masses: Masses,
    last_tick: u64,
    is_running: bool,
    sdl_context: Sdl,
    entities: Entities,
    positions: Positions,
}

impl Game {
    pub fn new() -> Game {
        info!("Setting up game.");

        let sdl_context = sdl2::init().unwrap();
        let mut render_system = render::renderer::RenderSystem::new(&sdl_context);
        let mut transform_system = core::transformsystem::TransformSystem::new();
        let mut entities = Entities::new();
        let mut masses = Masses::new();
        let mut positions = Positions::new();

        info!("Creating entities");

        for x in 0..25000 {
            let ent = square::make_square(&mut entities, &mut render_system, &mut transform_system, &mut masses, &mut positions);

            if x > 12500 {
                masses.register(&mut entities, ent, Mass{ value: 10f32 });
            }
        }

        Game {
            render_system: render_system,
            transform_system: transform_system,
            masses: masses,
            last_tick: time::precise_time_ns(),
            is_running: true,
            sdl_context: sdl_context,
            entities: entities,
            positions: positions,
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

        let ticks_as_seconds = (ticks as f32 / 1000000000f32) as f32;
//        
        let mut en = Entity::blank();
        en.id = self.entities.entities.len();
        self.masses.expand(en);
        self.positions.expand(en);

        for _ in 0..1000 {
            //self.transform_system.run(ticks);
            //core::transformsystem::move_right(ticks_as_seconds, &self.entities, &mut self.transform_system);
            core::transformsystem::move_right2(ticks_as_seconds, &mut self.transform_system);
        }
        for _ in 0..100 {
            //process_physics(ticks_as_seconds, &mut self.entities, &mut self.transform_system, &self.masses);
            process_physics2(ticks_as_seconds, &mut self.entities, &mut self.positions, &self.masses);
        }

        self.render_system.render(self.last_tick, &self.transform_system, &self.positions);
    }

    pub fn render(&mut self) {
    }

    pub fn close(&self) {
        info!("Goodbye!");
    }
}

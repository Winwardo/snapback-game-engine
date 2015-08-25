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
    pub mod component;
    pub mod system;
    pub mod physicssystem;
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

fn main() {
    env_logger::init().unwrap();
    let mut game = Game::new();
    game.run();
    game.close();
}

struct Game<'a> {
    render_system: render::renderer::RenderSystem<'a>,
    transform_system: core::transformsystem::TransformSystem,
    masses: Masses,
    last_tick: u64,
    is_running: bool,
    sdl_context: Sdl,
    entities: Entities,
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        info!("Setting up game.");

        let sdl_context = sdl2::init().unwrap();
        let mut render_system = render::renderer::RenderSystem::new(&sdl_context);
        let mut transform_system = core::transformsystem::TransformSystem::new();
        let mut masses = Masses { masses: vec![], };
        let mut entities = Entities::new();
/*
        for _ in 0..50 {
            entities.push(Box::new(square::Square::new(&render_system.sdl_renderer)));
        }
*/
        info!("Creating entities");

        //let s = Sprite::make(&mut render_system.sdl_renderer);
        //render_system.register(Rc::new(s));

        for _ in 0..25000 {
            square::make_square(&mut entities, &mut render_system, &mut transform_system, &mut masses);
        }

        Game {
            render_system: render_system,
            transform_system: transform_system,
            masses: masses,
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

        let ticks_as_seconds = (ticks as f32 / 1000000000f32) as f32;
//
        for _ in 0..100 {
            //self.transform_system.run(ticks);
            core::transformsystem::process_rotations(ticks_as_seconds, &self.entities, &mut self.transform_system);
        }
        self.render_system.render(self.last_tick, &self.transform_system);
        for _ in 0..100 {
            process_physics(ticks_as_seconds, &mut self.entities, &mut self.transform_system, &self.masses);
        }
    }

    pub fn render(&mut self) {
    }

    pub fn close(&self) {
        info!("Goodbye!");
    }
}

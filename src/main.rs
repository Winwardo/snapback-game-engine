#[macro_use] extern crate log;
extern crate env_logger;
extern crate sdl2;
extern crate time;

use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use std::thread;

mod render {
    pub mod renderer;
}

fn main() {
    env_logger::init().unwrap();
    let mut game = Game::new();
    game.run();
}


struct Game<'a> {
    renderSystem: render::renderer::RenderSystem<'a>,
    lastTick: u64,
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        info!("Setting up game.");

        Game {
            renderSystem: render::renderer::RenderSystem::new(),
            lastTick: time::precise_time_ns(),
        }
    }

    pub fn run(&mut self) {
        while true {
            self.tick();
            self.render();

            thread::sleep_ms(33);
        }
    }

    pub fn tick(&mut self) {
        self.lastTick = time::precise_time_ns();
    }

    pub fn render(&mut self) {
        self.renderSystem.render(self.lastTick);
    }
}




fn go() {
    /*
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();
    */

    //renderer.set_draw_color(Color::RGB(255, 0, 0));
    //renderer.clear();
    //renderer.present();

    let mut running = true;
    let mut tick = 0;


    /*
    let mut event_pump = sdl_context.event_pump().unwrap();

    while running {
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;

            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    running = false
                },
                _ => {}
            }
        }
        
        {
            // Update the window title.
            let mut window = renderer.window_mut().unwrap();

            let position = window.position();
            let size = window.size();
            let title = format!("Window - pos({}x{}), size({}x{}): {}", position.0, position.1, size.0, size.1, tick);
            window.set_title(&title);

            tick += 1;
        }

        let r:u8 = (tick*5) as u8;
        let g:u8 = (tick*7) as u8;
        let b:u8 = (tick*8) as u8;

        renderer.set_draw_color(Color::RGB(r,g,b));
        renderer.clear();
        renderer.present();

        thread::sleep_ms(33);
    }
    */
}

fn close() {
    info!("Goodbye!");
}


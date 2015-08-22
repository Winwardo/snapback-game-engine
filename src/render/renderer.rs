extern crate sdl2;

use sdl2::pixels::Color;
use std::boxed::Box;

static SCREEN_WIDTH: u32 = 1280;
static SCREEN_HEIGHT: u32 = 720;
static WINDOW_TITLE: &'static str = "Snapback engine";

pub struct RenderSystem<'a> {
   // window: sdl2::video::Window,
    sdlRenderer: sdl2::render::Renderer<'a>,
}


impl<'a> RenderSystem<'a> {
    pub fn new() -> RenderSystem<'a> {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window(WINDOW_TITLE, SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        //let q = Box::new(window);
        //let r = q.clone();

        let mut renderer = window.renderer().build().unwrap();

        RenderSystem {
            //window: window,
            sdlRenderer: renderer,
        }
    }

    pub fn render(&mut self, ticks: u64) {
        let r:u8 = (ticks*5) as u8;
        let g:u8 = (ticks*7) as u8;
        let b:u8 = (ticks*8) as u8;

        let title = format!("Snapback engine - {} ticks", ticks);
        //self.window.set_title(&title);

        self.sdlRenderer.set_draw_color(Color::RGB(r,g,b));
        self.sdlRenderer.clear();
        self.sdlRenderer.present();
    }
}
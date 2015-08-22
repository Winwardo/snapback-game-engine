extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::Sdl;

static SCREEN_WIDTH: u32 = 1280;
static SCREEN_HEIGHT: u32 = 720;
static WINDOW_TITLE: &'static str = "Snapback engine";

pub struct RenderSystem<'a> {
    sdl_renderer: sdl2::render::Renderer<'a>,
}

impl<'a> RenderSystem<'a> {
    pub fn new(sdl_context: &Sdl) -> RenderSystem<'a> {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window(WINDOW_TITLE, SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let renderer = window.renderer().build().unwrap();

        RenderSystem {
            sdl_renderer: renderer,
        }
    }

    pub fn render(&mut self, ticks: u64) {
        let r:u8 = (ticks*5) as u8;
        let g:u8 = (ticks*7) as u8;
        let b:u8 = (ticks*8) as u8;

        self.update_title(ticks);

        self.sdl_renderer.set_draw_color(Color::RGB(r,g,b));
        self.sdl_renderer.clear();
        self.sdl_renderer.present();
    }

    fn update_title(&mut self, ticks: u64) {

        let mut window = self.sdl_renderer.window_mut().unwrap();
        let title = format!("Snapback engine - {} ticks", ticks);
        window.set_title(&title);
    }
}
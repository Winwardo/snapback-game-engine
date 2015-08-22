extern crate sdl2;

use super::super::core::entity::*;
use super::renderable::*;

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
        let mut renderer = window.renderer().build().unwrap();

        renderer.set_draw_color(Color::RGB(70,80,160));

        RenderSystem {
            sdl_renderer: renderer,
        }
    }

    pub fn render(&mut self, tick: u64, entities: &Vec<Box<Entity>>) {
        self.update_title(tick);

        let mut renderables: Vec<Box<Renderable>> = Vec::new();

        for entity in entities {
            match entity.renderable() {
                Some(x) => renderables.push(x),
                None => {}
            }
        }

        for renderable in renderables {
            (*renderable).draw();
        }

        self.sdl_renderer.clear();
        self.sdl_renderer.present();
    }

    fn update_title(&mut self, tick: u64) {
        let mut window = self.sdl_renderer.window_mut().unwrap();
        let title = format!("Snapback engine - tick {}", tick);
        window.set_title(&title);
    }
}
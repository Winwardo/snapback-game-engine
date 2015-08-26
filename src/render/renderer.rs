extern crate sdl2;

use super::renderable::*;
use core::entity::*;
use core::sprite::*;

use sdl2::pixels::Color;
use sdl2::Sdl;

use core::transforms::position::*;
use core::transformsystem::*;
use core::system::*;

static SCREEN_WIDTH: u32 = 1280;
static SCREEN_HEIGHT: u32 = 720;
static WINDOW_TITLE: &'static str = "Snapback engine";

pub struct RenderSystem {
    pub sdl_renderer: sdl2::render::Renderer<'static>,
    drawables: Vec<Sprite>,
}

impl RenderSystem {
    pub fn new(sdl_context: &Sdl) -> RenderSystem {
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
            drawables: vec![],
        }
    }

    pub fn render(&mut self, tick: u64, transforms: &TransformSystem, positions: &Positions) {
        self.update_title(tick);
        self.sdl_renderer.clear();

        //info!("Start drawing all.");
        for drawable in self.drawables.iter() {
            //info!("Drawing.");
            drawable.draw(&mut self.sdl_renderer, &transforms, &positions);
        }

        self.sdl_renderer.present();
    }

    fn update_title(&mut self, tick: u64) {
        let mut window = self.sdl_renderer.window_mut().unwrap();
        let title = format!("Snapback engine - tick {}", tick);
        window.set_title(&title);
    }
}

impl System<Sprite> for RenderSystem {
    fn register(&mut self, sprite: Sprite) {
        self.drawables.push(sprite);
    }

    fn get(&self, entity: Entity) -> &Sprite {
        &self.drawables[entity.id]
    }

    fn get_mut(&mut self, entity: Entity) -> &mut Sprite {
        &mut self.drawables[entity.id]
    }

    fn run(&mut self, ticks: u64) {

    }
}
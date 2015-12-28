extern crate sdl2;

use core::world::*;

pub trait Renderable {
    fn draw<'a>(&self,
                sdl_renderer: &mut sdl2::render::Renderer<'a>,
                world: &World);
}

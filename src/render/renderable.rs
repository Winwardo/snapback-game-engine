extern crate sdl2;

use core::transformsystem::*;
use core::transforms::position::*;

pub trait Renderable {
    fn draw<'a>(&self,
                sdl_renderer: &mut sdl2::render::Renderer<'a>,
                transform_system: &TransformSystem,
                positions: &Positions);
}

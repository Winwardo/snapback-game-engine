extern crate sdl2;

use core::systems::transformsystem::*;
use core::transforms::position::*;
use core::world::*;

pub trait Renderable {
    fn draw<'a>(&self,
                sdl_renderer: &mut sdl2::render::Renderer<'a>,
                transform_system: &TransformSystem,
                world: &World);
}

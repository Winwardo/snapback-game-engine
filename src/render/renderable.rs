extern crate sdl2;

use core::transformsystem::*;

pub trait Renderable {
	fn draw<'a>(&self, sdl_renderer: &mut sdl2::render::Renderer<'a>, transform_system: &TransformSystem);
}

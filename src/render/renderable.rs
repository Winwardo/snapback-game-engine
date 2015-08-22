extern crate sdl2;

pub trait Renderable {
	fn draw<'a>(&self, sdl_renderer: &mut sdl2::render::Renderer<'a>);
}

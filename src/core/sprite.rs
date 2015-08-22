extern crate nalgebra as na;
extern crate sdl2;

use self::na::{Vec2};
use super::super::render::renderable::*;

use sdl2::rect::Rect;

pub struct Sprite {
	pub position: Vec2<f32>,
	pub scaling: Vec2<f32>,
	pub rotation: f32,
	pub texture: sdl2::render::Texture,
}

impl Renderable for Sprite {
	fn draw<'a>(&self, renderer: &mut sdl2::render::Renderer<'a>) {
	    // renderer.copy(&self.texture, None, Some(Rect::new_unwrap(self.position.x as i32, self.position.y as i32, 32, 32)));
    	renderer.copy_ex(&self.texture, None, Some(Rect::new_unwrap(self.position.x as i32, self.position.y as i32, 32, 32)), self.rotation as f64, None, (false, false));
	}
}

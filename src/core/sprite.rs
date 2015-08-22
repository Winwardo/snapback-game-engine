extern crate nalgebra as na;
extern crate sdl2;

use self::na::{Vec2};
use super::super::render::renderable::*;

use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;

pub struct Sprite {
	pub position: Vec2<f32>,
	pub scaling: Vec2<f32>,
	pub rotation: f32,
}

impl Renderable for Sprite {
	fn draw<'a>(&self, renderer: &mut sdl2::render::Renderer<'a>) {
		//info!("Rendering.");

		let mut texture = renderer.create_texture_streaming(PixelFormatEnum::RGB24, (256, 256)).unwrap();
	    // Create a red-green gradient
	    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
	        for y in (0..256) {
	            for x in (0..256) {
	                let offset = y*pitch + x*3;
	                buffer[offset + 0] = x as u8;
	                buffer[offset + 1] = y as u8;
	                buffer[offset + 2] = 0;
	            }
	        }
	    }).unwrap();

	    //renderer.clear();
	    renderer.copy(&texture, None, Some(Rect::new_unwrap(0, 0, 256, 256)));
	    renderer.copy_ex(&texture, None, Some(Rect::new_unwrap(450, 100, 256, 256)), 30.0, None, (false, false));
	}
}
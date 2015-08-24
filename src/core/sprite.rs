extern crate nalgebra as na;
extern crate sdl2;

use core::entity::*;
use self::na::{Vec2};
use super::super::render::renderable::*;
use sdl2::pixels::PixelFormatEnum;
use rand::Rng;
use std::rc::Rc;
use core::transformsystem::*;
use core::system::*;

use sdl2::rect::Rect;

pub struct Sprite {
	pub entity: u64,
	pub position: Vec2<f32>,
	pub scaling: Vec2<f32>,
	pub rotation: f32,
	pub texture: sdl2::render::Texture,
}

impl Renderable for Sprite {
	fn draw<'a>(&self, renderer: &mut sdl2::render::Renderer<'a>, transform_system: &TransformSystem) {
	    // renderer.copy(&self.texture, None, Some(Rect::new_unwrap(self.position.x as i32, self.position.y as i32, 32, 32)));
    	
	    // find rotation in transforms
	    let transform = transform_system.get(self.entity);

    	renderer.copy_ex(&self.texture, None, Some(Rect::new_unwrap(self.position.x as i32, self.position.y as i32, 32, 32)), transform.rotation as f64, None, (false, false));
	}
}

impl Sprite {
	pub fn make<'a>(entity: u64, renderer: &mut sdl2::render::Renderer<'a>) -> Sprite {
		let mut texture = renderer.create_texture_streaming(PixelFormatEnum::RGB24, (256, 256)).unwrap();
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

		Sprite {
			entity: entity,
			position: Vec2::new(100f32, 100f32),
			scaling: Vec2 {x: 1f32, y: 1f32},
			rotation: 0.0f32,
			texture: texture
		}
	}
}
extern crate nalgebra as na;
extern crate rand;
extern crate sdl2;

use core::entity::*;
use core::sprite::*;

use sdl2::pixels::PixelFormatEnum;
use self::na::{Vec2};
use super::super::render::renderable::*;
use self::rand::Rng;

pub struct Square {
	position: Vec2<f32>,
	ticks: u64,
	xoffset: f32,
	yoffset: f32,
	rotoffset: f32,
	sprite: Sprite,
}

impl Square {
	pub fn new<'a>(renderer: &sdl2::render::Renderer<'a>) -> Square {
		let mut rng = rand::thread_rng();

	    // Create a red-green gradient
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

		let sprite = Sprite {
			position: Vec2::new(100f32, 100f32),
			scaling: Vec2 {x: 1f32, y: 1f32},
			rotation: 0.0f32,
			texture: texture
		};

		Square {
			position: Vec2::new(100f32, 100f32),
			ticks: 0,
			xoffset: (rng.gen::<u32>() % 512) as f32,
			yoffset: (rng.gen::<u32>() % 512) as f32,
			rotoffset: (rng.gen::<u32>() % 360) as f32,
			sprite: sprite,
		}
	}
}

impl Entity for Square {
	fn update(&mut self, ticks: u64) {
		self.ticks += ticks;

		let norm = self.ticks as f32 / 100000000f32;
		let sin = norm.sin();
		let pos = 50f32 + 50f32*sin;

		self.sprite.position = Vec2::new(pos + self.xoffset, pos + self.yoffset);
		self.sprite.rotation = sin * 180f32 + 180f32 + self.rotoffset;
	}

	fn renderable(&self) -> Option<&Renderable> {
		Some(&self.sprite)
	}
}

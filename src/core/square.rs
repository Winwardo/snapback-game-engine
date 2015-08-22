extern crate nalgebra as na;

use core::entity::*;
use core::sprite::*;
use self::na::{Vec2};
use super::super::render::renderable::*;

pub struct Square {
	position: Vec2<f32>,
}

impl Square {
	pub fn new() -> Square {
		Square {
			position: Vec2::new(100f32, 100f32),
		}
	}
}

impl Entity for Square {
	fn update(&mut self, ticks: u64) {

	}

	fn renderable(&self) -> Option<Box<Renderable>> {
		Some(Box::new(Sprite {
			position: self.position.clone(),
			scaling: Vec2 {x: 1f32, y: 1f32},
			rotation: 0.0f32,
		}))
	}
}
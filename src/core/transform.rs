use core::component::*;
use nalgebra::{Vec2};

pub struct Transform {
	pub entity: u32,
	pub position: Vec2<f32>,
	pub scaling: Vec2<f32>,
	pub rotation: f32,
}

impl Component for Transform {
	fn entity(self) -> u32 {
		self.entity
	}
}

impl Transform {
	pub fn make_default(entity: u32) -> Transform {
		Transform {
			entity: entity,
			position: Vec2{ x: 0f32, y: 0f32 },
			scaling: Vec2{ x: 0f32, y: 0f32 },
			rotation: 0f32,
		}
	}
}
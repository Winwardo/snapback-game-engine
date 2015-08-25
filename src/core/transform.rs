use core::component::*;
use core::entity::*;
use nalgebra::{Vec2};

pub struct Transform {
	pub entity: Entity,
	pub position: Vec2<f32>,
	pub scaling: Vec2<f32>,
	pub rotation: f32,
}

impl Component for Transform {
	fn entity(self) -> Entity {
		self.entity
	}
}

impl Transform {
	pub fn make_default(entity: Entity) -> Transform {
		Transform {
			entity: entity,
			position: Vec2{ x: 0f32, y: 0f32 },
			scaling: Vec2{ x: 0f32, y: 0f32 },
			rotation: 0f32,
		}
	}
}
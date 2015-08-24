use core::component::*;
use core::entity2::*;
use nalgebra::{Vec2};

struct Transform {
	pub parent: Entity2,
	pub position: Vec2<f32>,
	pub scaling: Vec2<f32>,
	pub rotation: f32,
}

impl Component for Transform {
	fn entity(self) -> Entity2 {
		self.parent
	}
}
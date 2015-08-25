extern crate nalgebra as na;

use core::entity::*;
use core::transform::*;
use core::system::*;

pub struct TransformSystem {
	transforms: Vec<Transform>,
}

impl TransformSystem {
	pub fn new() -> TransformSystem {
		TransformSystem { transforms: Vec::new() }
	}
}

impl System<Transform> for TransformSystem {
	fn register(&mut self, transform: Transform) {
		self.transforms.push(transform);
	}

	fn get(&self, entity: Entity) -> &Transform {
		&self.transforms[entity.id]
	}

	fn get_mut(&mut self, entity: Entity) -> &mut Transform {
		&mut self.transforms[entity.id]
	}

	fn run(&mut self, ticks: u64) {
		for t in self.transforms.iter_mut() {
			t.rotation += 90f32 * ticks as f32 / 1000000000f32;
		}
	}
}
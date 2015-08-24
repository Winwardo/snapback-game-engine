extern crate nalgebra as na;

use core::transform::*;
use self::na::{Vec2};
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

	fn get(&self, entity: u64) -> &Transform {
		&self.transforms[0]
	}
}
extern crate nalgebra as na;

use core::transform::*;
use self::na::{Vec2};

pub struct TransformSystem {
	transforms: Vec<Transform>,
}

impl TransformSystem {
	pub fn new() -> TransformSystem {
		TransformSystem { transforms: Vec::new() }
	}

	pub fn register(&mut self, transform: Transform) {
		self.transforms.push(transform);
	}

	pub fn get(&self, entity: u64) -> &Transform {
		&self.transforms[0]
	}
}
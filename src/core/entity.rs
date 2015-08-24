extern crate time;

use std::rc::Rc;

pub struct Entity2 {
	id: u64,
}

impl Entity2 {
	pub fn new() -> Entity2 {
		Entity2 { id: time::precise_time_ns() }
	}
}
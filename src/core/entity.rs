extern crate time;

pub struct Entity {
	id: u64,
}

impl Entity {
	pub fn new() -> Entity {
		Entity { id: time::precise_time_ns() }
	}
}
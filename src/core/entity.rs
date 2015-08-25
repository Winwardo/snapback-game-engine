use std::rc::Rc;

#[derive(Copy, Clone)]
pub struct Entity {
	pub id: u32,
}

/// Master of all existing entities.
pub struct Entities {
	active: Vec<bool>
}

impl Entities {
	pub fn new() -> Entities {
		Entities {
			active: Vec::with_capacity(1024),
		}
	}

	pub fn create_entity(&mut self) -> Entity {
		let new_id = self.first_inactive_space();

		if new_id >= self.active.len() {
			self.active.push(true);
		} else {
			self.active[new_id] = true;
		}

		Entity { id: new_id as u32, }
	}

	fn first_inactive_space(&self) -> usize {
		let active_len = self.active.len();
		for i in 0..active_len {
			if self.active[i] == false {
				return i;
			}
		}

		return active_len;
	}
}
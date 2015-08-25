use std::rc::Rc;

pub struct Entity {
	pub id: u32,
}

/// Master of all existing entities.
pub struct Entities {
	entities: Vec<Rc<Entity>>,
	active: Vec<bool>
}

impl Entities {
	pub fn new() -> Entities {
		Entities {
			entities: Vec::with_capacity(1024),
			active: Vec::with_capacity(1024),
		}
	}

	pub fn create_entity(&mut self) -> Rc<Entity> {
		let new_id = self.first_inactive_space();
		let result = Rc::new(
			Entity { id: new_id as u32, });

		if new_id >= self.entities.len() {
			self.entities.push(result.clone());
			self.active.push(true);
		} else {
			self.entities[new_id] = result.clone();
			self.active[new_id] = true;
		}

		result
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
use std::rc::Rc;

pub struct Entity {
	id: u32,
}

/// Master of all existing entities.
struct Entities {
	entities: Vec<Rc<Entity>>,
	active: Vec<bool>
}

impl Entities {
	pub fn new(self) -> Rc<Entity> {
		let new_id = self.first_inactive_space();

		Rc::new(
			Entity { id: new_id, })
	}

	fn first_inactive_space(self) -> u32 {
		for i in 0..self.active.len() {
			if self.active[i] == false {
				return i as u32;
			}
		}

		return 0;
	}
}
use std::rc::Rc;

bitflags! {
    flags ComponentFlag: u32 {
        const C_UNUSED    		= 0b00000001,
        const C_SPRITE       	= 0b00000010,
        const C_MASS	     	= 0b00000100,
        const C_TRANSFORM    	= 0b00001000,
    }
}

#[derive(Copy, Clone)]
pub struct Entity {
	pub id: usize,
	flags: ComponentFlag,
}

impl Entity {
	pub fn has_flags(self, flags: ComponentFlag) -> bool {
		self.flags.contains(flags)
	}
}

/// Master of all existing entities.
pub struct Entities {
	pub entities: Vec<Entity>,
}

impl Entities {
	pub fn new() -> Entities {
		Entities {
			entities: Vec::with_capacity(1024),
		}
	}

	pub fn create_entity(&mut self, flags: ComponentFlag) -> Entity {
		let new_id = self.first_inactive_space();
		let entity = Entity { id: new_id, flags: flags };

		if new_id >= self.entities.len() {
			self.entities.push(entity);
		} else {
			self.entities[new_id] = entity;
		}

		entity
	}

	fn first_inactive_space(&self) -> usize {
		let active_len = self.entities.len();
		for i in 0..active_len {
			if self.entities[i].flags.is_empty() {
				return i;
			}
		}

		return active_len;
	}
}
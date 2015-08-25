use std::rc::Rc;

#[derive(Copy, Clone)]
pub struct Entity {
	pub id: usize,
}


bitflags! {
    flags ComponentFlag: u32 {
        const C_UNUSED    		= 0b00000001,
        const C_SPRITE       	= 0b00000010,
        const C_MASS	     	= 0b00000100,
        const C_TRANSFORM    	= 0b00001000,
    }
}

/// Master of all existing entities.
pub struct Entities {
	pub components: Vec<ComponentFlag>,
}

impl Entities {
	pub fn new() -> Entities {
		Entities {
			components: Vec::with_capacity(1024),
		}
	}

	pub fn create_entity(&mut self, flags: ComponentFlag) -> Entity {
		let new_id = self.first_inactive_space();

		if new_id >= self.components.len() {
			self.components.push(flags);
		} else {
			self.components[new_id] = flags;
		}

		Entity { id: new_id, }
	}

	fn first_inactive_space(&self) -> usize {
		let active_len = self.components.len();
		for i in 0..active_len {
			if self.components[i].is_empty() {
				return i;
			}
		}

		return active_len;
	}

	pub fn get_flags_for_entity(&mut self, entity: Entity) -> &ComponentFlag {
		&self.components[entity.id]
	}

	pub fn components(&self) -> &Vec<ComponentFlag> {
		&self.components
	}
}
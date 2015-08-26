#![feature(macro_rules)]

use core::entity::*;

pub trait Component {
	fn entity(self) -> Entity;
}

pub trait Components<T> {
	fn entities_mut(&mut self) -> &mut Vec<Entity>;
	fn components_mut(&mut self) -> &mut Vec<T>;
	fn entities(&self) -> &Vec<Entity>;
	fn components(&self) -> &Vec<T>;	

	fn register(&mut self, entity: Entity, component: T) {
		self.components_mut().push(component);

		let mut entities = self.entities_mut();

		while entities.len() <= entity.id {
			entities.push(Entity::blank());
		}
		entities[entity.id] = entity;
	}

	fn get_mut(&mut self, entity: Entity) -> &mut T {
		return &mut self.components_mut()[entity.id];
	}

	fn get(&self, entity: Entity) -> &T {
		return &self.components()[entity.id];
	}
}

macro_rules! components {
	( $x:ident, $y:ty ) => {
		impl Components<$y> for Positions {
			fn entities_mut(&mut self) -> &mut Vec<Entity> { &mut self.entities }
			fn components_mut(&mut self) -> &mut Vec<$y> { &mut self.$x }
			fn entities(&self) -> &Vec<Entity> { &self.entities }
			fn components(&self) -> &Vec<$y> { &self.$x }
		}
	}
}

macro_rules! component_super {
	( $struct_name:ty, $vec_name:ident, $value:ident ) => {
		pub struct $vec_name {
			pub $value: Vec<$struct_name>,
			pub entities: Vec<Entity>,
		}

		impl $vec_name {
			pub fn new() -> $vec_name {
				$vec_name {
					$value: Vec::new(),
					entities: Vec::new(),
				}
			}
		}

		impl Components<$struct_name> for $vec_name {
			fn entities_mut(&mut self) -> &mut Vec<Entity> { &mut self.entities }
			fn components_mut(&mut self) -> &mut Vec<$struct_name> { &mut self.$value }
			fn entities(&self) -> &Vec<Entity> { &self.entities }
			fn components(&self) -> &Vec<$struct_name> { &self.$value }
		}
	}
}
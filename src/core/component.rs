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

	fn register(&mut self, entities_master: &mut Entities, entity: Entity, component: T,);

	fn get_mut(&mut self, entity: Entity) -> &mut T {
		return &mut self.components_mut()[entity.id];
	}

	fn get(&self, entity: Entity) -> &T {
		return &self.components()[entity.id];
	}
}

macro_rules! components {
	( $struct_name:ty, $vec_name:ident, $comp_flag:ident ) => {
		pub struct $vec_name {
			components: Vec<$struct_name>,
			pub entities: Vec<Entity>,
		}

		impl $vec_name {
			pub fn new() -> $vec_name {
				$vec_name {
					components:	Vec::new(),
					entities:	Vec::new(),
				}
			}
		}

		impl Components<$struct_name> for $vec_name {
			#[inline] fn entities_mut(&mut self) -> &mut Vec<Entity> { &mut self.entities }
			#[inline] fn components_mut(&mut self) -> &mut Vec<$struct_name> { &mut self.components }
			#[inline] fn entities(&self) -> &Vec<Entity> { &self.entities }
			#[inline] fn components(&self) -> &Vec<$struct_name> { &self.components }


			fn register(&mut self, entities_master: &mut Entities, entity: Entity, component: $struct_name) {
				self.components_mut().push(component);
				let mut entities = self.entities_mut();

				while entities.len() <= entity.id {
					entities.push(Entity::blank());
				}
				entities[entity.id] = entity;

				entities_master.set_flag(entity, $comp_flag);
			}
		}
	}
}
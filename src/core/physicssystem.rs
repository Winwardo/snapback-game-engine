use core::entity::*;
use core::transformsystem::*;
use core::system::*;

pub fn process_physics(ticks: f32, entities: &mut Entities, transforms: &mut TransformSystem, masses: &Masses) {
	for entity in &entities.entities {
		if entity.has_flags(C_MASS | C_TRANSFORM) {
			let mut transform = transforms.get_mut(*entity);
			let mass = masses.get(*entity);

			transform.position.y += mass as f32 * ticks;
		}
	}
}

pub struct Masses {
	pub masses: Vec<u8>,
}

impl Masses {
	pub fn register(&mut self, entity: Entity, mass: u8) {
		if entity.id >= self.masses.len() {
			self.masses.push(mass);
		} else {
			self.masses[entity.id] = mass;
		}
	}

	pub fn get(&self, entity: Entity) -> u8 {
		self.masses[entity.id]
	}
}
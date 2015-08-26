use core::entity::*;
use core::transformsystem::*;
use core::transforms::position::*;
use core::system::*;
use core::component::*;
use core::mass::*;


pub fn process_physics(ticks: f32, entities: &mut Entities, positions: &mut Positions, masses: &Masses) {
	for x in entities.with_flags(C_MASS | C_POSITION) {
		let mut position = positions.get_mut(*x);
		let mass = masses.get(*x);

		position.value.y += mass.value * ticks;
	};
}

pub fn process_ylimit(ticks: f32, positions: &mut Positions) {
	for position in positions.components_mut() {
		if position.value.y > 128f32 {
			position.value.y = 128f32;
		}
	}
}
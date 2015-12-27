use core::entity::*;
use core::transforms::position::*;
use core::system::*;
use core::component::*;
use core::mass::*;
use core::world::*;

pub fn process_physics(ticks_as_seconds: f32,
                       entities: &mut Entities,
                       positions: &mut Positions,
                       masses: &Masses) {
    for x in entities.with_flags(C_MASS | C_POSITION) {
        let mut position = positions.get_mut(*x);
        let mass = masses.get(*x);

        position.value.y += mass.value * ticks_as_seconds;
    }
}

pub fn process_physics2(ticks_as_seconds: f32, world: &mut World) {}

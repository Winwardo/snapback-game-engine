use core::entity::*;
use core::transforms::position::*;
use core::system::*;
use core::component::*;
use core::mass::*;
use core::world::*;
use core::times::tick::*;

pub fn process_physics(ticks: Ticks, world: &mut World) {
    for x in world.entities.with_flags(C_MASS | C_TRANSFORM) {
        let mass = world.masses_non()
                        .get(x)
                        .clone();
        let mut transform = world.transforms().get_mut(x);

        transform.position.y += mass.value * ticks;
    }
}

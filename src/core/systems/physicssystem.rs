use core::entity::*;
use core::transforms::position::*;
use core::system::*;
use core::component::*;
use core::mass::*;
use core::world::*;
use core::times::tick::*;

pub fn process_physics(ticks: Ticks, world: &mut World) {
    for x in world.entities.with_flags(C_MASS | C_TRANSFORM) {
        apply_floor_gravity(ticks, world, x);
    }
}

fn apply_floor_gravity(ticks: Ticks, world: &mut World, entity: Entity) {
    let mass = world.masses_non()
                    .get(entity)
                    .clone();
    let mut transform = world.transforms().get_mut(entity);

    transform.position.y += mass.value * ticks;
}

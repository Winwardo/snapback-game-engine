use std::ops::Add;
use std::ops::Mul;

use core::entity::*;
use core::system::*;
use core::component::*;
use core::world::*;
use core::times::tick::*;

const GRAVITY_RATE: f32 = 9.81 * 100f32;

pub fn process_physics(ticks: Ticks, world: &mut World) {
    for x in world.entities.with_flags(C_MASS | C_TRANSFORM | C_MOVEMENT) {
        apply_floor_gravity(ticks, world, x);
        apply_movement(ticks, world, x);
        apply_bounce(ticks, world, x);
        apply_transforms_from_movement(ticks, world, x);
        rotate_based_on_x(ticks, world, x);
    }
}

fn rotate_based_on_x(ticks: Ticks, world: &mut World, entity: Entity) {
    let mut transform = world.transforms().get_mut(entity);

    let position = transform.position;
    transform.rotation += (position.x / 4f32) * ticks;
}

fn apply_floor_gravity(ticks: Ticks, world: &mut World, entity: Entity) {
    let mut movement = world.movements().get_mut(entity);

    movement.velocity.y += GRAVITY_RATE * ticks;
}

fn apply_movement(ticks: Ticks, world: &mut World, entity: Entity) {
    let mut movement = world.movements().get_mut(entity);

    movement.velocity = movement.velocity.add(movement.acceleration.mul(ticks));
}

fn apply_transforms_from_movement(ticks: Ticks, world: &mut World, entity: Entity) {
    let movement = world.movements
                        .get(entity)
                        .clone();
    let mut transform = world.transforms().get_mut(entity);

    transform.position = transform.position.add(movement.velocity.mul(ticks));
}

fn apply_bounce(_: Ticks, world: &mut World, entity: Entity) {
    let transform = world.transforms.get(entity).clone();
    let mut movement = world.movements().get_mut(entity);

    if transform.position.y > 200f32 && movement.velocity.y > 1f32 {
        movement.velocity.y = -movement.velocity.y * 0.99f32;

        movement.velocity.x += transform.rotation / 32f32;
    }
}

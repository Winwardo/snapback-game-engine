extern crate nalgebra as na;

use core::component::*;
use core::entity::*;
use core::transform::*;
use core::system::*;
use core::world::*;
use core::times::tick::*;

pub struct TransformSystem {
    pub transforms: Vec<Transform>,
}

impl TransformSystem {
    pub fn new() -> TransformSystem {
        TransformSystem { transforms: Vec::new() }
    }

    pub fn move_all(&mut self, ticks: Ticks, world: &mut World, amount: f32) {
        for position in world.positions().components_mut() {
            position.value.x += amount;
        }
    }
}

impl System<Transform> for TransformSystem {
    fn register(&mut self, transform: Transform) {
        self.transforms.push(transform);
    }

    fn get(&self, entity: Entity) -> &Transform {
        &self.transforms[entity.id]
    }

    fn get_mut(&mut self, entity: Entity) -> &mut Transform {
        &mut self.transforms[entity.id]
    }

    fn tick(&mut self, world: &mut World, ticks: Ticks) {
        self.move_all(ticks, world, 1f32);
        // for t in self.transforms.iter_mut() {
            // t.rotation += 90f32 * ticks as f32 / 1000000000f32;
        // }
    }
}

pub fn process_rotations(ticks: f32, entities: &Entities, transforms: &mut TransformSystem) {
    for (entity, transform) in entities.entities.iter().zip(transforms.transforms.iter_mut()) {
        if entity.has_flags(C_TRANSFORM) {
            // let mut transform2 = transforms.get_mut(*entity);

            // let mut q = transform.rotation;
            // unsafe{ q += 1f32; };

            transform.rotation += 90f32 * ticks as f32;
        }
    }
}

pub fn move_right(ticks: f32, entities: &Entities, transforms: &mut TransformSystem) {
    for (entity, transform) in entities.entities.iter().zip(transforms.transforms.iter_mut()) {
        if entity.has_flags(C_TRANSFORM) {
            // let mut transform2 = transforms.get_mut(*entity);

            // let mut q = transform.rotation;
            // unsafe{ q += 1f32; };

            // transform.position.x += 1f32;
        }
    }
}

pub fn move_right2(ticks: f32, world: &mut World) {
    // for (entity, transform) in entities.entities.iter().zip(transforms.transforms.iter_mut()) {
    // 	if entity.has_flags(C_TRANSFORM) {
    // let mut transform2 = transforms.get_mut(*entity);

    // let mut q = transform.rotation;
    // unsafe{ q += 1f32; };

    // println!("go");


    println!("go");
    for position in world.positions.components_mut() {
        println!("yeehaw");
        position.value.x += 1f32;
        println!("{:?}", position.value.x);
        // position.value.x = 0f32;
        // println!("x: {}", transform.position.x);
    }
    // }
}

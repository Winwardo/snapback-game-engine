extern crate nalgebra as na;

use core::component::*;
use core::entity::*;
use core::transforms::transform::*;
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
        for transform in world.transforms.components_mut() {
            transform.position.x += 0.1f32;
            transform.rotation += 1f32;
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

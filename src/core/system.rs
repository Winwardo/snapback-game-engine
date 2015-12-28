use core::entity::*;
use core::times::tick::*;
use core::world::*;

pub trait System<T> {
    fn register(&mut self, component: T);
    fn get(&self, entity: Entity) -> &T;
    fn get_mut(&mut self, entity: Entity) -> &mut T;
    fn tick(&mut self, world: &mut World, ticks: Ticks);
}

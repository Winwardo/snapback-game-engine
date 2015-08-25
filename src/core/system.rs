use core::entity::*;

pub trait System<T> {
	fn register(&mut self, component: T);
	fn get(&self, entity: Entity) -> &T;
	fn get_mut(&mut self, entity: Entity) -> &mut T;
	fn run(&mut self, ticks: u64);
}
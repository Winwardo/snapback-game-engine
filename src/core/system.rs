pub trait System<T> {
	fn register(&mut self, component: T);
	fn get(&self, entity: u64) -> &T;
}
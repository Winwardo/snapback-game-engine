pub trait System<T> {
	fn register(&mut self, component: T);
	fn get(&self, entity: u32) -> &T;
	fn run(&mut self, ticks: u64);
}
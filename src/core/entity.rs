use super::super::render::renderable::*;

pub trait Entity {
	fn update(&mut self, ticks: u64);
	fn renderable(&self) -> Option<Box<Renderable>>;
}
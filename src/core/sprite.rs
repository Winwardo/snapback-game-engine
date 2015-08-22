extern crate nalgebra as na;

use self::na::{Vec2};
use super::super::render::renderable::*;

pub struct Sprite {
	pub position: Vec2<f32>,
	pub scaling: Vec2<f32>,
	pub rotation: f32,
}

impl Renderable for Sprite {
	fn draw(&self) {
		info!("Rendering.");
	}
}
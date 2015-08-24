use render::renderer::*;
use core::entity::*;
use core::sprite::*;
use core::transformsystem::*;
use core::transform::*;
use std::rc::Rc;

extern crate time;

pub fn make_square<'a>(renderSystem: &mut RenderSystem<'a>, transformSystem: &mut TransformSystem) -> u64 {
	let entity = time::precise_time_ns();

	let sprite = Sprite::make(entity, &mut renderSystem.sdl_renderer);
	renderSystem.register(sprite);

	let mut transform = Transform::make_default(entity);
	transform.rotation = 45f32;
	transformSystem.register(transform);

	entity
}
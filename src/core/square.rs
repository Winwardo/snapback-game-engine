use render::renderer::*;
use core::sprite::*;
use core::transformsystem::*;
use core::transform::*;
use core::system::*;

extern crate time;

pub fn make_square<'a>(render_system: &mut RenderSystem<'a>, transform_system: &mut TransformSystem) -> u64 {
	let entity = time::precise_time_ns();

	let sprite = Sprite::make(entity, &mut render_system.sdl_renderer);
	render_system.register(sprite);

	let mut transform = Transform::make_default(entity);
	transform.rotation = 45f32;
	transform_system.register(transform);

	entity
}
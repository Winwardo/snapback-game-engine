use render::renderer::*;
use core::sprite::*;
use core::transformsystem::*;
use core::transform::*;
use core::system::*;
use core::entity::*;

extern crate time;

pub fn make_square<'a>(entities: &mut Entities, render_system: &mut RenderSystem<'a>, transform_system: &mut TransformSystem) -> u32 {
	let entity_ = entities.create_entity();
	let entity = entity_.id;

	let sprite = Sprite::make(entity, &mut render_system.sdl_renderer);
	render_system.register(sprite);

	let mut transform = Transform::make_default(entity);
	transform.rotation = 45f32 + entity_.id as f32;
	transform.position.x += entity as f32;
	transform_system.register(transform);

	entity
}
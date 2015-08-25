use render::renderer::*;
use core::sprite::*;
use core::transformsystem::*;
use core::transform::*;
use core::system::*;
use core::entity::*;
use core::physicssystem::*;

extern crate time;

pub fn make_square<'a>(entities: &mut Entities, render_system: &mut RenderSystem<'a>, transform_system: &mut TransformSystem, masses: &mut Masses) -> Entity {
	let entity = entities.create_entity(C_SPRITE | C_TRANSFORM | C_MASS);

	let sprite = Sprite::make(entity, &mut render_system.sdl_renderer);
	render_system.register(sprite);

	let mut transform = Transform::make_default(entity);
	transform.rotation = 45f32 + entity.id as f32;
	transform.position.x += entity.id as f32;
	transform_system.register(transform);

	let mass = entity.id as u8;
	masses.register(entity, mass);

	entity
}
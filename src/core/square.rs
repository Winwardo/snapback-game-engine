use render::renderer::*;
use core::sprite::*;
use core::transformsystem::*;
use core::transform::*;
use core::transforms::position::*;
use core::system::*;
use core::entity::*;
use core::physicssystem::*;
use nalgebra::{Vec2};
use core::component::*;
use core::mass::*;

extern crate time;

pub fn make_square<'a>(
		entities: &mut Entities,
		render_system: &mut RenderSystem,
		transform_system: &mut TransformSystem,
		masses: &mut Masses,
		positions: &mut Positions,
		) -> Entity {
	let entity = entities.create_entity(C_SPRITE | C_TRANSFORM);

	let sprite = Sprite::make(entity, &mut render_system.sdl_renderer);
	render_system.register(sprite);

	let mut transform = Transform::make_default(entity);
	transform.rotation = 45f32 + entity.id as f32;
	transform.position.x += entity.id as f32;
	transform_system.register(transform);

	positions.register(entity, Position{ value: Vec2 { x: entity.id as f32, y: 0f32 }, });

	entity
}

pub fn make_square_mass<'a>(
		entities: &mut Entities,
		render_system: &mut RenderSystem,
		transform_system: &mut TransformSystem,
		masses: &mut Masses,
		positions: &mut Positions,
		) -> Entity {
	let entity = entities.create_entity(C_SPRITE | C_TRANSFORM | C_MASS);

	let sprite = Sprite::make(entity, &mut render_system.sdl_renderer);
	render_system.register(sprite);

	let mut transform = Transform::make_default(entity);
	transform.rotation = 45f32 + entity.id as f32;
	transform.position.x += entity.id as f32;
	transform_system.register(transform);

	masses.register(entity, Mass { value: entity.id as f32, });

	positions.register(entity, Position{ value: Vec2 { x: entity.id as f32, y: 0f32 }, });

	entity
}
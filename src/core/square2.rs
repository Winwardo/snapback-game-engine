use render::renderer::*;
use core::entity::*;
use core::sprite::*;

pub fn make_square<'a>(renderSystem: &mut RenderSystem<'a>) -> Entity2 {
	let entity = Entity2::new();

	let sprite = Sprite::make(&entity, &mut renderSystem.sdl_renderer);
	renderSystem.register(sprite);

	entity
}
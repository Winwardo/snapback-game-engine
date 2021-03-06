use render::renderer::*;
use core::sprite::*;
use core::transforms::transform::*;
use core::system::*;
use core::entity::*;
use core::component::*;
use core::physics::mass::*;
use core::physics::movement::*;
use core::world::*;

extern crate time;

pub fn make_square<'a>(world: &mut World, render_system: &mut RenderSystem) -> Entity {
    let entity = world.entities.create_entity(C_SPRITE | C_TRANSFORM);

    let sprite = Sprite::make(entity, &mut render_system.sdl_renderer);
    render_system.register(sprite);

    let m = &mut world.entities;
    {
        let mut transform = Transform::make_default(entity);
        transform.position.x = entity.id as f32 * 32f32;
        world.transforms.register(m, entity, transform);
    }
    world.masses.register(m, entity, Mass { value: 10f32 });
    world.movements.register(m, entity, Movement::zero());

    entity
}

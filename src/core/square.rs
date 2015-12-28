use render::renderer::*;
use core::sprite::*;
use core::systems::transformsystem::*;
use core::transforms::transform::*;
use core::transforms::position::*;
use core::transforms::rotation::*;
use core::system::*;
use core::entity::*;
use nalgebra::Vec2;
use core::component::*;
use core::mass::*;
use core::world::*;

extern crate time;

pub fn make_square<'a>(world: &mut World,
                       render_system: &mut RenderSystem,
                       transform_system: &mut TransformSystem)
                       -> Entity {
    let entity = world.entities.create_entity(C_SPRITE | C_TRANSFORM);

    let sprite = Sprite::make(entity, &mut render_system.sdl_renderer);
    render_system.register(sprite);

    let mut transform = Transform::make_default(entity);
    transform.rotation = 45f32 + entity.id as f32;
    transform_system.register(transform);



    let m = &mut world.entities;

    world.transforms.register(m, entity, Transform::make_default(entity));
    world.masses.register(m, entity, Mass { value: 10f32 });
    entity
}

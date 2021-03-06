extern crate nalgebra as na;
extern crate sdl2;

use core::entity::*;
use super::super::render::renderable::*;
use sdl2::pixels::PixelFormatEnum;
use core::system::*;
use core::component::*;
use core::world::*;

use sdl2::rect::Rect;

pub struct Sprite {
    pub entity: Entity,
    pub texture: sdl2::render::Texture,
}

impl Renderable for Sprite {
    fn draw<'a>(&self, renderer: &mut sdl2::render::Renderer<'a>, world: &World) {
        // find rotation in transforms
        let transform = world.transforms.get(self.entity);
        // let position = world.positions.get(self.entity);
        let position = transform.position;
        let rotation = transform.rotation;

        renderer.copy_ex(&self.texture,
                         None,
                         Some(Rect::new_unwrap(position.x as i32, position.y as i32, 32, 32)),
                         rotation as f64,
                         None,
                         (false, false));
    }
}

impl Sprite {
    pub fn make<'a>(entity: Entity, renderer: &mut sdl2::render::Renderer<'a>) -> Sprite {
        let mut texture = renderer.create_texture_streaming(PixelFormatEnum::RGB24, (16, 16))
                                  .unwrap();
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
                   for y in 0..16 {
                       for x in 0..16 {
                           let offset = y * pitch + x * 3;
                           buffer[offset + 0] = x as u8;
                           buffer[offset + 1] = y as u8;
                           buffer[offset + 2] = 0;
                       }
                   }
               })
               .unwrap();

        Sprite {
            entity: entity,
            texture: texture,
        }
    }
}

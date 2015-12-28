use core::entity::*;
use core::component::*;
use core::transforms::rotation::*;
use core::transforms::position::*;
use nalgebra::Vec2;

#[derive(Copy, Clone)]
pub struct Transform {
    pub entity: Entity,
    pub scaling: Vec2<f32>,
    pub rotation: Rotation,
    pub position: Position,
}

impl Transform {
    pub fn make_default(entity: Entity) -> Transform {
        Transform {
            entity: entity,
            scaling: Vec2 { x: 0f32, y: 0f32 },
            rotation: 0f32,
            position: Position { x: 0f32, y: 0f32 },
        }
    }
}

components!(Transform, Transforms, C_TRANSFORM);

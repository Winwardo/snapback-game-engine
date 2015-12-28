use core::entity::*;
use core::component::*;
use nalgebra::Vec2;

#[derive(Copy, Clone)]
pub struct Movement {
    pub velocity: Vec2<f32>,
    pub acceleration: Vec2<f32>,
}

impl Movement {
    pub fn zero() -> Movement {
        Movement {
            velocity: Vec2::new(0f32, 0f32),
            acceleration: Vec2::new(0f32, 0f32),
        }
    }
}

components!(Movement, Movements, C_MOVEMENT);

use core::entity::*;
use core::component::*;
use nalgebra::Vec2;

pub struct Position {
    pub value: Vec2<f32>,
}

components!(Position, Positions, C_POSITION);

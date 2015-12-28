use core::entity::*;
use core::component::*;

pub struct Rotation {
    pub value: f32,
}

components!(Rotation, Rotations, C_ROTATION);
 

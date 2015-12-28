use core::entity::*;
use core::component::*;

#[derive(Copy, Clone)]
pub struct Mass {
    pub value: f32,
}

components!(Mass, Masses, C_MASS);

use core::entity::*;
use core::transforms::position::*;
use core::mass::*;

pub struct World {
    pub entities: Entities,
    pub positions: Positions,
    pub masses: Masses,
}

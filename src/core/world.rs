use core::entity::*;
use core::transforms::position::*;
use core::mass::*;

pub struct World {
    pub entities: Entities,
    pub positions: Positions,
    pub masses: Masses,
}

impl World {
    pub fn entities(&self) -> &Entities {
        &self.entities
    }
    pub fn entities_mut(&mut self) -> &mut Entities {
        &mut self.entities
    }
    pub fn positions(&self) -> &Positions {
        &self.positions
    }
    pub fn positions_mut(&mut self) -> &mut Positions {
        &mut self.positions
    }
    pub fn masses(&self) -> &Masses {
        &self.masses
    }
    pub fn masses_mut(&mut self) -> &mut Masses {
        &mut self.masses
    }
}

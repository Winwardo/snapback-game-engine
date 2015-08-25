use core::entity::*;

pub trait Component {
	fn entity(self) -> Entity;
}
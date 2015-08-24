use core::entity2::*;

pub trait Component {
	fn entity(self) -> Entity2;
}
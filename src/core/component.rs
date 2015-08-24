use core::entity::*;
use std::rc::Rc;

pub trait Component {
	fn entity(self) -> u64;
}
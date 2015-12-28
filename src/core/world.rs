use core::entity::*;
use core::transforms::position::*;
use core::physics::mass::*;
use core::physics::movement::*;
use core::transforms::rotation::*;
use core::transforms::transform::*;

macro_rules! struct_world {
    ($($element: ident: $ty: ty),*) => {
        pub struct World {
        	$(pub $element: $ty),*
        }
    }
}

macro_rules! impl_world {
    ($element: ident: $ty: ty) => {
        impl World {
    		pub fn $element(&mut self) -> &mut $ty {
    			&mut self.$element
    		}
        }
    }
}

macro_rules! impl_world_long {
    ($($element: ident: $ty: ty),*) => {
        $(
        	impl_world!($element: $ty);
        )*
    }
}

macro_rules! make_world {
	// https://stackoverflow.com/questions/32289605/
	// how-do-i-write-a-wrapper-for-a-macro-without-repeating-the-rules
    ($($tts:tt)*) => {
        struct_world!($($tts)*);
    	impl_world_long!($($tts)*);
    }
}

make_world!(
	entities: Entities, 
	masses: Masses,
	transforms: Transforms,
	movements: Movements
);

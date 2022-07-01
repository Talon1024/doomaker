//! Misc. utility functions
use glam::Vec2;

mod angle {
	use std::cmp::Ordering::{self, *};
	use derive_deref::*;
	#[derive(Debug, Clone, Copy, Deref, PartialEq)]
	pub struct Angle(pub f32);
	impl PartialOrd for Angle {
		fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
			// Assuming all angles are in radians
			let result = self.0.partial_cmp(&other.0);
			if self.0 > 0. && other.0 > 0. {
				result
			} else if self.0 <= 0. && other.0 > 0. {
				Some(Greater)
			} else if self.0 >= 0. && other.0 < 0. {
				Some(Less)
			} else {
				result
			}
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;
		use std::error::Error;
		#[test]
		fn angle_sort() -> Result<(), Box<dyn Error>> {
			let mut angles = Box::<[Angle]>::from([
				Angle(-3.125),
				Angle(3.125),
				Angle(2.75),
				Angle(-2.75),
				Angle(-0.125),
				Angle(0.125),
				Angle(1.75),
				Angle(-2.25),
			]);
			let expected = Box::<[Angle]>::from([
				Angle(0.125),
				Angle(1.75),
				Angle(2.75),
				Angle(3.125),
				Angle(-3.125),
				Angle(-2.75),
				Angle(-2.25),
				Angle(-0.125),
			]);
			// The PartialOrd implementation for Angle does not return
			// None, so this should be safe.
			angles.sort_unstable_by(|a, b| {a.partial_cmp(b).unwrap()});
			assert_eq!(angles, expected);
			Ok(())
		}
	}
}
pub use angle::Angle;

pub fn vec2_angle(vec: Vec2) -> Angle {
	#[cfg(micromath)]
	use micromath::F32;

	cfg_if::cfg_if! {
		if #[cfg(micromath)] {
			Angle(F32(vec.y).atan2(F32(vec.x)).0)
		} else {
			Angle(vec.y.atan2(vec.x))
		}
	}
}
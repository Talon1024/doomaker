//! Misc. utility functions
use glam::Vec2;

pub fn vec2_angle(vec: Vec2) -> f32 {
	#[cfg(micromath)]
	use micromath::F32;

	cfg_if::cfg_if! {
		if #[cfg(micromath)] {
			F32(vec.y).atan2(F32(vec.x)).0
		} else {
			vec.y.atan2(vec.x)
		}
	}
}

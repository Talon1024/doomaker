use std::cmp::{Ordering, Ordering::{Equal, Greater, Less}};
// X, Y
#[derive(PartialEq, PartialOrd, Clone, Debug, Copy, Default)]
pub struct MapVertex(f32, f32);
pub struct MapVertexEx {
	pub p: MapVertex,
	pub floor_height: Option<f32>,
	pub ceiling_height: Option<f32>
}

impl From<&[f32]> for MapVertex {
	fn from(v: &[f32]) -> MapVertex {
		MapVertex(
			v.get(0).cloned().expect("No X position!"),
			v.get(1).cloned().expect("No Y position!"),
		)
	}
}

impl From<&[f32]> for MapVertexEx {
	fn from(v: &[f32]) -> MapVertexEx {
		MapVertexEx {
			p: MapVertex::from(v),
			floor_height: v.get(2).cloned(),
			ceiling_height: v.get(3).cloned()
		}
	}
}

impl Eq for MapVertex {}

impl Ord for MapVertex {
	fn cmp(&self, other: &Self) -> Ordering {
		if other.1 > self.1 {
			if other.0 > self.0 {
				Greater
			} else {
				Less
			}
		} else if other.0 > self.0 {
			Less
		} else {
			Equal
		}
	}
}

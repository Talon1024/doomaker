use std::cmp::Ordering::{self, Equal, Greater, Less};
use crate::vector::Vector2;
// X, Y
#[derive(PartialEq, PartialOrd, Clone, Debug, Copy, Default)]
pub struct MapVertex {
	pub p: Vector2
}
pub struct MapVertexEx {
	pub p: MapVertex,
	pub floor_height: Option<f32>,
	pub ceiling_height: Option<f32>
}

impl From<Vector2> for MapVertex {
	fn from(v: Vector2) -> MapVertex {
		MapVertex { p: v }
	}
}

impl From<&[f32]> for MapVertex {
	fn from(v: &[f32]) -> MapVertex {
		MapVertex { p: Vector2::from(v) }
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

impl Eq for MapVertex{}
impl Ord for MapVertex {
	fn cmp(&self, other: &Self) -> Ordering {
		if other.p.x() == self.p.x() {
			if other.p.y() > self.p.y() {
				Greater
			} else {
				Less
			}
		} else if other.p.x() > self.p.x() {
			Less
		} else {
			Equal
		}
	}
}

pub fn midpoint(vertices: &(Vector2, Vector2)) -> Vector2 {
	let (a, b) = vertices;
	(a + b) / 2.0
}
pub fn edge_length(vertices: &(Vector2, Vector2)) -> f32 {
	let (a, b) = vertices;
	let relative_position = b - a;
	relative_position.dot(&relative_position).sqrt()
}

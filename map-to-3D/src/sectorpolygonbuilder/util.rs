use glam::Vec2;
use crate::edge::{Edge, EdgeVertexIndex};
use super::Angle;

fn point_in_polygon(point: Vec2, polygon: &Vec<Vec2>) -> bool {
	// Based on https://wrf.ecse.rpi.edu/Research/Short_Notes/pnpoly.html
	let mut inside = false;
	for i in 0..polygon.len() {
		let j = if i == 0 { polygon.len() - 1 } else { i - 1 };
		let vi = polygon[i];
		let vj = polygon[j];
		if (
			(vi.y > point.y) != (vj.y > point.y)) && (
			point.x < (vj.x - vi.x) * (point.y - vi.y) / (vj.y - vi.y) + vi.x
		) {
			inside = !inside;
		}
	}
	inside
}

pub(super) fn edge_in_polygon(
	edge: &Edge,
	polygon: &[EdgeVertexIndex],
	map_vertices: &[Vec2]
) -> bool {
	let a = map_vertices[edge.lo()];
	let b = map_vertices[edge.hi()];
	let midpoint = (a + b) / 2.;
	let polygon: Vec<Vec2> = polygon.iter()
		.map(|&index| map_vertices[index])
		.collect();
	point_in_polygon(midpoint, &polygon)
}

pub(super) fn angle_between(
	p1: Vec2,
	p2: Vec2,
	center: Vec2,
	clockwise: bool
) -> Angle {
	#[cfg(micromath)]
	use micromath::F32;
	let ac = p1 - center;
	let bc = p2 - center;

	let ang = ac.angle_between(bc) *
		if clockwise {-1.} else {1.};

	Angle(ang)
}

pub(super) fn vec2_angle(vec: Vec2) -> f32 {
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

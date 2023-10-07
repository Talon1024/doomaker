//! Line segments and intersection calculation
use glam::Vec2;
use ahash::RandomState;
use std::collections::HashSet;
use fixed::types::I32F32;

// A line segment
#[derive(Debug, Clone, Copy)]
pub struct Segment(pub Vec2, pub Vec2);

impl Segment {
	pub fn intersection(&self, other: Segment) -> Option<Intersection> {
		// Thanks to https://replit.com/@thehappycheese/linetools#LineTools/line_tools.py
		// and his YouTube video: https://youtu.be/5FkOO1Wwb8w
		{ // If any of the four points are equal, the segments are connected.
			let mut points = HashSet::<[I32F32; 2], RandomState>::default();
			points.insert([I32F32::saturating_from_num(self.0.x), I32F32::saturating_from_num(self.0.y)]);
			points.insert([I32F32::saturating_from_num(self.1.x), I32F32::saturating_from_num(self.1.y)]);
			points.insert([I32F32::saturating_from_num(other.0.x), I32F32::saturating_from_num(other.0.y)]);
			points.insert([I32F32::saturating_from_num(other.1.x), I32F32::saturating_from_num(other.1.y)]);
			if points.len() < 4 {
				// I don't think a connection counts as an intersection
				return None;
			}
		}
		let ab = self.1 - self.0;
		let cd = other.1 - other.0;
		let ab_cross_cd = ab.perp_dot(cd);
		if ab_cross_cd == 0. { // Lines are parallel
			// TODO: Check for collinear segments
			Some(Intersection::Collinear)
		} else {
			let ac = other.0 - self.0;
			let ab_factor = ac.perp_dot(cd) / ab_cross_cd;
			let cd_factor = -ab.perp_dot(ac) / ab_cross_cd;
			if ab_factor < 0. || ab_factor > 1. || cd_factor < 0. || cd_factor > 1. {
				None
			} else {
				Some(Intersection::Normal(self.0 + ab * ab_factor))
			}
		}
	}
}

pub enum Intersection {
	Normal(Vec2),
	Collinear
}

impl Intersection {
	pub fn split(&self, a: Segment, b: Segment) -> Vec<Segment> {
		let verts = [a.0, a.1, b.0, b.1];
		match self {
			Intersection::Normal(i) => {
				// Both segments intersect at the intersection point
				verts.into_iter().map(|v| Segment(v, *i)).collect()
			},
			Intersection::Collinear => {
				if verts.into_iter().map(|v| v.y).all(|v| v == verts[0].y) {
					// Go from left to right
					let mut verts = verts.clone();
					verts.sort_by(|&a, &b| b.x.partial_cmp(&a.x).unwrap());
					verts.windows(2).map(|w| Segment(w[0], w[1])).collect()
				} else {
					// Go from top to bottom
					let mut verts = verts.clone();
					verts.sort_by(|&a, &b| b.y.partial_cmp(&a.y).unwrap());
					verts.windows(2).map(|w| Segment(w[0], w[1])).collect()
				}
			},
		}
	}
}

/*
4,8 16,11 collinear 8,9 12,10
4,8 16,11 collinear 12,10 8,9
16,11 4,8 collinear 8,9 12,10
16,11 4,8 collinear 12,10 8,9
4,8 16,11 collinear 8,9 -4,6
4,8 16,11 collinear -4,6 8,9
16,11 4,8 collinear 8,9 -4,6
16,11 4,8 collinear -4,6 8,9
4,8 16,11 nontersec -8,5 0,7
16,11 4,8 nontersec -8,5 0,7
*/

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn intersection_simplea() {
		use glam::const_vec2;
		let pa = Segment(Vec2::new(2., 2.), Vec2::new(-2., -2.));
		let pb = Segment(Vec2::new(-2., 2.), Vec2::new(2., -2.));
		let intersection_point = pa.intersection(pb);
		assert!(intersection_point.is_some());

		let Intersection::Normal(intersection_point) = intersection_point.unwrap() else {
			panic!("Intersection is collinear for some reason!");
		};
		let intersection_point = format!("{:.3} {:.3}", intersection_point.x, intersection_point.y);
		let expected = const_vec2!([0., 0.]);
		let expected = format!("{:.3} {:.3}", expected.x, expected.y);
		assert_eq!(expected, intersection_point);
	}

	#[test]
	fn intersection_simpleb() {
		use glam::const_vec2;
		let pa = Segment(Vec2::new(4., 4.), Vec2::new(0., 0.));
		let pb = Segment(Vec2::new(0., 4.), Vec2::new(4., 0.));
		let intersection_point = pa.intersection(pb);
		assert!(intersection_point.is_some());

		let Intersection::Normal(intersection_point) = intersection_point.unwrap() else {
			panic!("Intersection is collinear for some reason!");
		}; 
		let intersection_point = format!("{:.3} {:.3}", intersection_point.x, intersection_point.y);
		let expected = const_vec2!([2., 2.]);
		let expected = format!("{:.3} {:.3}", expected.x, expected.y);
		assert_eq!(expected, intersection_point);
	}

	#[test]
	fn intersection_complex() {
		use glam::const_vec2;
		let pa = Segment(Vec2::new(7., 1.), Vec2::new(9., 2.));
		let pb = Segment(Vec2::new(7., 3.), Vec2::new(9., 1.));
		let intersection_point = pa.intersection(pb);
		assert!(intersection_point.is_some());

		let Intersection::Normal(intersection_point) = intersection_point.unwrap() else {
			panic!("Intersection is collinear for some reason!");
		}; 
		let intersection_point = format!("{:.3} {:.3}", intersection_point.x, intersection_point.y);
		let expected = const_vec2!([8.333333333, 1.666666666]);
		let expected = format!("{:.3} {:.3}", expected.x, expected.y);
		assert_eq!(expected, intersection_point);
	}

	#[test]
	fn intersection_none() {
		// No intersection
		let pa = Segment(Vec2::new(-2., -2.), Vec2::new(1., 3.));
		let pb = Segment(Vec2::new(1., 1.), Vec2::new(4., -2.));
		let intersection_point = pa.intersection(pb);
		assert!(intersection_point.is_none());
	}

	#[test]
	fn intersection_parallel() {
		// Parallel segments
		let pa = Segment(Vec2::new(0., 1.), Vec2::new(4., -2.));
		let pb = Segment(Vec2::new(-3., -2.), Vec2::new(1., -5.));
		let intersection_point = pa.intersection(pb);
		assert!(intersection_point.is_none());
	}

	#[test]
	fn intersection_collinear() {
		// Collinear segments
		let pa = Segment(Vec2::new(0., 1.), Vec2::new(4., -2.));
		let pb = Segment(Vec2::new(-3., -2.), Vec2::new(1., -5.));
		let intersection_point = pa.intersection(pb);
		assert!(intersection_point.is_none());
	}

	#[test]
	fn intersection_connected() {
		// Connected segments
		let pa = Segment(Vec2::new(3., 6.), Vec2::new(4., 2.));
		let pb = Segment(Vec2::new(4., 2.), Vec2::new(8., -1.));
		let intersection_point = pa.intersection(pb);
		assert!(intersection_point.is_none());
	}
}

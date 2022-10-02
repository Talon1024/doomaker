//! Line segments and intersection calculation
use glam::Vec2;

// A line segment
#[derive(Debug, Clone, Copy)]
pub struct Segment(pub Vec2, pub Vec2);

pub enum SegmentIntersection {
	Normal(Vec2),
	Collinear([Option<Vec2>; 2])
}

impl Segment {
	pub fn intersection(&self, other: Segment) -> Option<Vec2> {
		// Thanks to https://replit.com/@thehappycheese/linetools#LineTools/line_tools.py
		// and his YouTube video: https://youtu.be/5FkOO1Wwb8w
		let ab = self.1 - self.0;
		let cd = other.1 - other.0;
		let ab_cross_cd = ab.perp_dot(cd);
		if ab_cross_cd == 0. { // Lines are parallel
			None
		} else {
			let ac = other.0 - self.0;
			let ab_factor = ac.perp_dot(cd) / ab_cross_cd;
			let cd_factor = -ab.perp_dot(ac) / ab_cross_cd;
			if ab_factor < 0. || ab_factor > 1. || cd_factor < 0. || cd_factor > 1. {
				None
			} else {
				Some(self.0 + ab * ab_factor)
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn intersection_a() {
		use glam::const_vec2;
		let pa = Segment(Vec2::new(2., 2.), Vec2::new(-2., -2.));
		let pb = Segment(Vec2::new(-2., 2.), Vec2::new(2., -2.));
		let intersection_point = pa.intersection(pb);
		assert!(intersection_point.is_some());

		let expected = const_vec2!([0., 0.]);
		let intersection_point = intersection_point.map(|v| format!("{:.3} {:.3}", v.x, v.y)).unwrap();
		let expected = format!("{:.3} {:.3}", expected.x, expected.y);
		assert_eq!(expected, intersection_point);
	}

	#[test]
	fn intersection_b() {
		use glam::const_vec2;
		let pa = Segment(Vec2::new(4., 4.), Vec2::new(0., 0.));
		let pb = Segment(Vec2::new(0., 4.), Vec2::new(4., 0.));
		let intersection_point = pa.intersection(pb);
		assert!(intersection_point.is_some());

		let expected = const_vec2!([2., 2.]);
		let intersection_point = intersection_point.map(|v| format!("{:.3} {:.3}", v.x, v.y)).unwrap();
		let expected = format!("{:.3} {:.3}", expected.x, expected.y);
		assert_eq!(expected, intersection_point);
	}

	#[test]
	fn intersection_c() {
		use glam::const_vec2;
		let pa = Segment(Vec2::new(7., 1.), Vec2::new(9., 2.));
		let pb = Segment(Vec2::new(7., 3.), Vec2::new(9., 1.));
		let intersection_point = pa.intersection(pb);
		assert!(intersection_point.is_some());

		let expected = const_vec2!([8.333333333, 1.666666666]);
		let intersection_point = intersection_point.map(|v| format!("{:.3} {:.3}", v.x, v.y)).unwrap();
		let expected = format!("{:.3} {:.3}", expected.x, expected.y);
		assert_eq!(expected, intersection_point);
	}

	#[test]
	fn intersection_d() {
		// No intersection
		let pa = Segment(Vec2::new(-2., -2.), Vec2::new(1., 3.));
		let pb = Segment(Vec2::new(1., 1.), Vec2::new(4., -2.));
		let intersection_point = pa.intersection(pb);
		assert!(intersection_point.is_none());
	}

	#[test]
	fn intersection_e() {
		// Parallel segments
		let pa = Segment(Vec2::new(0., 1.), Vec2::new(4., -2.));
		let pb = Segment(Vec2::new(-3., -2.), Vec2::new(1., -5.));
		let intersection_point = pa.intersection(pb);
		assert!(intersection_point.is_none());
	}
}

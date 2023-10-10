//! Line segments and intersection calculation
use glam::Vec2;

// A line segment
#[derive(Debug, Clone, Copy)]
pub struct Segment(pub Vec2, pub Vec2);

impl Segment {
	pub fn intersection(&self, other: Segment) -> Option<Intersection> {
		// Thanks to https://replit.com/@thehappycheese/linetools#LineTools/line_tools.py
		// and his YouTube video: https://youtu.be/5FkOO1Wwb8w
		{ // If any of the four points are equal, the segments are connected.
			let ab = [self.0, self.1];
			let cd = [other.0, other.1];
			// Both points of the first segment equal both points of the second
			if ab.iter().all(|av| cd.iter().any(|bv| av == bv)) {
				return Some(Intersection::Same);
			// Either point of the first segment equals either point of the second
			} else if ab.iter().any(|av| cd.iter().any(|bv| av == bv)) {
				return Some(Intersection::Connected);
			}
		}
		let ab = self.1 - self.0;
		let cd = other.1 - other.0;
		let ac = other.0 - self.0;
		let ab_cross_cd = ab.perp_dot(cd);
		if ab_cross_cd == 0. { // Lines are parallel
			let ab_slope = self.slope();
			let cd_slope = other.slope();
			// Are points c or d between a and b? If so, there is a collinear
			// intersection.
			if ab_slope.is_some() && cd_slope.is_some() && ab_slope == cd_slope {
				let ad = other.1 - self.0;
				let ca = self.0 - other.0;
				let cb = self.1 - other.0;
				let vectors = [(ac, ab), (ad, ab), (ca, cd), (cb, cd)];
				vectors.into_iter()
					.map(|(pt, div)| {
						let v = pt / div;
						// It is unlikely that v.x will not equal v.y, since
						// the dividend and divisor are on the same line
						match ab_slope.unwrap() {
							Slope::Vertical { x: _x } => v.y,
							Slope::Horizontal { y: _y } => v.x,
							Slope::Sloped { m: _m, b: _b } => v.x,
						}
					})
					.any(|f| (0.0..1.0).contains(&f))
					.then_some(Intersection::Collinear)
			} else {
				None
			}
		} else {
			let ab_factor = ac.perp_dot(cd) / ab_cross_cd;
			let cd_factor = -ab.perp_dot(ac) / ab_cross_cd;
			(!(ab_factor < 0.) && !(ab_factor > 1.) &&
			 !(cd_factor < 0.) && !(cd_factor > 1.))
			.then_some(Intersection::Normal(self.0 + ab * ab_factor))
		}
	}
	fn slope(&self) -> Option<Slope> {
		if self.0 == self.1 {
			// Highly unlikely
			None
		} else if self.0.x == self.1.x {
			Some(Slope::Vertical { x: self.0.x })
		} else if self.0.y == self.1.y {
			Some(Slope::Horizontal { y: self.0.y })
		} else {
			let m = {
				let (a, b) = match self {
					Segment(a, b) if a.x < b.x => (*a, *b),
					Segment(a, b) => (*b, *a)
				};
				let diff = b - a;
				diff.y / diff.x
			};
			// y = mx + b
			// y - b = mx
			// -b = mx - y
			// b = -(mx - y)
			let b = -(m * self.0.x - self.0.y);
			Some(Slope::Sloped { m, b })
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Slope {
	Vertical {x: f32},
	Horizontal {y: f32},
	Sloped {m: f32, b: f32}
}

/// An intersection between two line segments
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Intersection {
	/// The lines cross each other at this point
	Normal(Vec2),
	/// The lines are collinear
	Collinear,
	/// The lines are connected at either point
	Connected,
	/// The two lines are the same
	Same
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
			Intersection::Connected => {
				vec![a, b]
			},
			Intersection::Same => {
				vec![a]
			}
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
		let pa = Segment(Vec2::new(2., 2.), Vec2::new(-2., -2.));
		let pb = Segment(Vec2::new(-2., 2.), Vec2::new(2., -2.));
		let intersection_point = pa.intersection(pb);

		let Intersection::Normal(intersection_point) = intersection_point.unwrap() else {
			panic!("Intersection is not normal for some reason!");
		};
		let intersection_point = format!("{:.3} {:.3}", intersection_point.x, intersection_point.y);
		let expected = Vec2::from_array([0., 0.]);
		let expected = format!("{:.3} {:.3}", expected.x, expected.y);
		assert_eq!(expected, intersection_point);
	}

	#[test]
	fn intersection_simpleb() {
		let pa = Segment(Vec2::new(4., 4.), Vec2::new(0., 0.));
		let pb = Segment(Vec2::new(0., 4.), Vec2::new(4., 0.));
		let intersection_point = pa.intersection(pb);

		let Intersection::Normal(intersection_point) = intersection_point.unwrap() else {
			panic!("Intersection is not normal for some reason!");
		}; 
		let intersection_point = format!("{:.3} {:.3}", intersection_point.x, intersection_point.y);
		let expected = Vec2::from_array([2., 2.]);
		let expected = format!("{:.3} {:.3}", expected.x, expected.y);
		assert_eq!(expected, intersection_point);
	}

	#[test]
	fn intersection_complex() {
		let pa = Segment(Vec2::new(7., 1.), Vec2::new(9., 2.));
		let pb = Segment(Vec2::new(7., 3.), Vec2::new(9., 1.));
		let intersection_point = pa.intersection(pb);

		let Intersection::Normal(intersection_point) = intersection_point.unwrap() else {
			panic!("Intersection is not normal for some reason!");
		}; 
		let intersection_point = format!("{:.3} {:.3}", intersection_point.x, intersection_point.y);
		let expected = Vec2::from_array([8.333333333, 1.666666666]);
		let expected = format!("{:.3} {:.3}", expected.x, expected.y);
		assert_eq!(expected, intersection_point);
	}

	#[test]
	fn intersection_none() {
		// Intersection is outside of these segments' bounds
		let pa = Segment(Vec2::new(-2., -2.), Vec2::new(1., 4.)); // 2x + 2
		let pb = Segment(Vec2::new(1., 1.), Vec2::new(4., -2.)); // -x + 2
		let intersection_point = pa.intersection(pb);
		assert!(intersection_point.is_none());
	}

	#[test]
	fn intersection_parallel() {
		// Parallel segments
		let pa = Segment(Vec2::new(0., 1.), Vec2::new(4., -2.)); // -0.75x + 1
		let pb = Segment(Vec2::new(-3., -2.), Vec2::new(1., -5.)); // -0.75x - 4.25
		let intersection_point = pa.intersection(pb);
		assert_eq!(intersection_point, None);
	}

	#[test]
	fn intersection_collinear() {
		// Segment pb goes from the midpoint of segment pa to below the x-axis
		// Both points are on this line: y = -0.5x + 4
		let pa = Segment(Vec2::new(2., 3.), Vec2::new(6., 1.));
		let pb = Segment(Vec2::new(4., 2.), Vec2::new(10., -1.));
		let intersection_point = pa.intersection(pb);
		assert_eq!(intersection_point, Some(Intersection::Collinear));
	}

	#[test]
	fn intersection_collinear_shorter() {
		// Segment pb is within segment pa
		// y = -0.5x + 4
		let pa = Segment(Vec2::new(2., 3.), Vec2::new(6., 1.));
		let pb = Segment(Vec2::new(4., 2.), Vec2::new(5., 1.5));
		let intersection_point = pa.intersection(pb);
		assert_eq!(intersection_point, Some(Intersection::Collinear));
	}

	#[test]
	fn intersection_collinear_shorter_reversed() {
		// Segment pa is within segment pb
		// y = -0.5x + 4
		let pa = Segment(Vec2::new(4., 2.), Vec2::new(5., 1.5));
		let pb = Segment(Vec2::new(2., 3.), Vec2::new(6., 1.));
		let intersection_point = pa.intersection(pb);
		assert_eq!(intersection_point, Some(Intersection::Collinear));
	}

	#[test]
	fn intersection_collinear_spaced() {
		// Both segments are on this line, but do not touch each other:
		// y = -0.5x + 20
		let pa = Segment(Vec2::new(4., 18.), Vec2::new(6., 17.));
		let pb = Segment(Vec2::new(8., 16.), Vec2::new(12., 14.));
		let intersection_point = pa.intersection(pb);
		assert_eq!(intersection_point, None);
	}

	#[test]
	fn intersection_collinear_horizontal() {
		// Both segments are on this line:
		// y = 5
		let pa = Segment(Vec2::new(4., 5.), Vec2::new(6., 5.));
		let pb = Segment(Vec2::new(8., 5.), Vec2::new(5., 5.));
		let intersection_point = pa.intersection(pb);
		assert_eq!(intersection_point, Some(Intersection::Collinear));
	}

	#[test]
	fn intersection_collinear_horizontal_spaced() {
		// Both segments are on this line, but do not touch each other:
		// y = 5
		let pa = Segment(Vec2::new(4., 5.), Vec2::new(6., 5.));
		let pb = Segment(Vec2::new(8., 5.), Vec2::new(10., 5.));
		let intersection_point = pa.intersection(pb);
		assert_eq!(intersection_point, None);
	}

	#[test]
	fn intersection_collinear_vertical() {
		// Both segments are on this line:
		// y = 5
		let pa = Segment(Vec2::new(5., 4.), Vec2::new(5., 6.));
		let pb = Segment(Vec2::new(5., 8.), Vec2::new(5., 5.));
		let intersection_point = pa.intersection(pb);
		assert_eq!(intersection_point, Some(Intersection::Collinear));
	}

	#[test]
	fn intersection_collinear_vertical_spaced() {
		// Both segments are on this line, but do not touch each other:
		// x = 5
		let pa = Segment(Vec2::new(5., 4.), Vec2::new(5., 6.));
		let pb = Segment(Vec2::new(5., 8.), Vec2::new(5., 10.));
		let intersection_point = pa.intersection(pb);
		assert_eq!(intersection_point, None);
	}

	#[test]
	fn intersection_connected() {
		// Connected segments
		let pa = Segment(Vec2::new(3., 6.), Vec2::new(4., 2.));
		let pb = Segment(Vec2::new(4., 2.), Vec2::new(8., -1.));
		let intersection_point = pa.intersection(pb);
		assert_eq!(intersection_point, Some(Intersection::Connected));
	}

	#[test]
	fn intersection_same() {
		// Connected segments
		let pa = Segment(Vec2::new(3., 6.), Vec2::new(4., 2.));
		let pb = Segment(Vec2::new(4., 2.), Vec2::new(3., 6.));
		let intersection_point = pa.intersection(pb);
		assert_eq!(intersection_point, Some(Intersection::Same));
	}
}

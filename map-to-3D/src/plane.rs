//! Sector floor/ceiling planes
use glam::{Vec2, Vec3};
/// The geometric plane of a sector floor/ceiling
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Plane {
	/// A flat sector plane, represented by a single floating point value,
	/// which is the height of the plane
	Flat( /// Height
		f32
	),
	/// A sloped sector plane, represented internally by the ABCD terms of a
	/// plane equation (`Ax + By + Cz + D = 0`). A, B, and C are the XYZ
	/// components of the plane's normal vector.
	Sloped(
		/// A
		f32,
		/// B
		f32,
		/// C
		f32,
		/// D
		f32)
}

impl Default for Plane {
	fn default() -> Self {
		Plane::Flat(0.)
	}
}

impl Plane {
	/// Get the height of the sector plane at the given position
	/// 
	/// # Examples
	/// 
	/// ```
	/// use map_to_3D::plane::Plane;
	/// use glam::Vec2;
	/// 
	/// let flat_plane = Plane::Flat(16.);
	/// let pos = Vec2::new(16., 16.);
	/// assert_eq!(flat_plane.z_at(pos), 16.);
	/// ```
	pub fn z_at(&self, pos: Vec2) -> f32 {
		match self {
			Plane::Flat(height) => *height,
			Plane::Sloped(
				a, b, c, d
			) => {
				// https://github.com/Talon1024/jsdoom/blob/5c3ca7553b/src/convert/3DMapBuilder.ts#L650
				// Also https://github.com/coelckers/gzdoom/blob/7ba5a74f2e/src/gamedata/r_defs.h#L356
				let x = pos.x;
				let y = pos.y;
				let dividend = a * x + b * y + d;
				dividend / -c
			}
		}
	}

	/// Calculate the normal vector of this sector plane
	/// 
	/// If `reverse` is true, inverts the normal vector
	/// 
	/// # Examples
	/// 
	/// With a flat plane:
	/// ```
	/// use map_to_3D::plane::Plane;
	/// use glam::Vec3;
	/// 
	/// let expected: Vec3 = Vec3::from_array([0.0f32, 0.0f32, 1.0f32]);
	/// let actual = Plane::Flat(5.).normal(false);
	/// assert_eq!(expected, actual);
	/// ```
	/// 
	/// With a slope defined by the equation `z = .5x + .5y`:
	/// ```
	/// use map_to_3D::plane::Plane;
	/// use glam::Vec3;
	/// 
	/// let plane = {
	/// 	// Based on z = .5x + .5y + 5
	/// 	let z: f32 = 1.;
	/// 	let y = 0.5 * z;
	/// 	let x = 0.5 * z;
	/// 	let l = (x * x + y * y + z * z).sqrt();
	/// 	let a = -x / l;
	/// 	let b = -y / l;
	/// 	let c = z / l;
	/// 	(a, b, c)
	/// };
	/// // Use strings because of the inaccuracies caused by how real numbers
	/// // are represented
	/// let expected = format!(
	/// 	"{:.3} {:.3} {:.3} ",
	/// 	plane.0, plane.1, plane.2);
	/// let actual = Plane::from_triangle(
	/// 	Vec3::new(16., 16., 21.),
	/// 	Vec3::new(-16., 16., 5.),
	/// 	Vec3::new(-16., -16., -11.)
	/// ).normal(false);
	/// let actual: String = (0..3).map(|co| format!("{:.3} ", actual[co])).collect();
	/// assert_eq!(expected, actual);
	/// ```
	pub fn normal(&self, reverse: bool) -> Vec3 {
		match self {
			Plane::Flat(_) => Vec3::new(0., 0., if reverse {-1.} else {1.}),
			Plane::Sloped(a, b, c, _) => {
				if reverse {
					// I don't know if this is correct
					Vec3::new(-*a, -*b, -*c)
				} else {
					Vec3::new(*a, *b, *c)
				}
			}
		}
	}

	/// Create a `Plane` from three points (a triangle)
	/// 
	/// # Examples
	/// 
	/// With a flat plane:
	/// ```
	/// use map_to_3D::plane::Plane;
	/// use glam::Vec3;
	/// 
	/// let expected = Plane::Flat(5.);
	/// let actual = Plane::from_triangle(
	/// 	Vec3::new(16., 16., 5.),
	/// 	Vec3::new(-16., 16., 5.),
	/// 	Vec3::new(-16., -16., 5.)
	/// );
	/// assert_eq!(expected, actual);
	/// ```
	/// 
	/// With a slope defined by the equation `z = .5x + .5y`:
	/// ```
	/// use map_to_3D::plane::Plane;
	/// use glam::Vec3;
	/// 
	/// let plane = {
	/// 	// Based on z = .5x + .5y + 5
	/// 	let z: f32 = 1.;
	/// 	let y = 0.5 * z;
	/// 	let x = 0.5 * z;
	/// 	let l = (x * x + y * y + z * z).sqrt();
	/// 	let a = -x / l;
	/// 	let b = -y / l;
	/// 	let c = z / l;
	/// 	let d = 5. * -c;
	/// 	(a, b, c, d)
	/// };
	/// // Use strings because of the inaccuracies caused by how real numbers
	/// // are represented
	/// let expected = format!(
	/// 	"{:.3} {:.3} {:.3} {:.3}",
	/// 	plane.0, plane.1, plane.2, plane.3);
	/// let actual = Plane::from_triangle(
	/// 	Vec3::new(16., 16., 21.),
	/// 	Vec3::new(-16., 16., 5.),
	/// 	Vec3::new(-16., -16., -11.)
	/// );
	/// match actual {
	/// 	Plane::Sloped(a, b, c, d) => {
	/// 		let actual = format!("{:.3} {:.3} {:.3} {:.3}", a, b, c, d);
	/// 		assert_eq!(expected, actual);
	/// 	},
	/// 	Plane::Flat(h) => {
	/// 		panic!("The plane should be sloped!");
	/// 	}
	/// }
	/// ```
	pub fn from_triangle(
		v1: Vec3,
		v2: Vec3,
		v3: Vec3,
	) -> Plane {
		if v1.z == v2.z && v1.z == v3.z {
			Plane::Flat(v1.z)
		} else {
			// Diff point 1 and 2
			let d1 = v2 - v1;
			// Diff point 1 and 3
			let d2 = v3 - v1;
			// Calculate ABC
			let abc = d1.cross(d2).normalize_or_zero();
			let (a, b, c) = (abc.x, abc.y, abc.z);
			// Calculate D
			// Ax + By + Cz + D = 0
			// Ax + By + Cz = -D
			// D = -(Ax + By + Cz)
			let d = -(a * v1.x + b * v1.y + c * v1.z);
			Plane::Sloped(a, b, c, d)
		}
	}

	/// Calculate the intersection point between two planes on a line, if they
	/// intersect.
	pub fn intersection(&self, a: Vec2, b: Vec2, other: &Plane) -> Option<Vec3> {
		// Pretend the slopes are lines in 2D space, with the two points on them
		// being a and b, and that the X of point a is at 0. Calculate slope
		// and y-intercept for both lines
		let xy = b - a;
		let line_len = xy.length();
		// Self slope, self y-intercept, other slope, other y-intercept
		let (ss, sy, os, oy) = {
			let zas = self.z_at(a);
			let zbs = self.z_at(b);
			let zao = other.z_at(a);
			let zbo = other.z_at(b);
			((zbs - zas) / line_len, zas,
			 (zbo - zao) / line_len, zao)
		};
		// https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_line_equations
		// ptx = (d - c) / (a - b)
		let ptx = (oy - sy) / (ss - os);
		if ptx.is_nan() || ptx <= 0. || ptx > line_len {
			None
		} else {
			let xy = ptx / line_len * xy + a;
			let ptz = ss * ptx + sy;
			Some(xy.extend(ptz))
		}
	}

	/// Convert a flat plane to a sloped plane
	pub fn to_sloped(self) -> Plane {
		match self {
			Plane::Flat(h) => { Plane::Sloped(0., 0., 1., h) }
			_ => self
		}
	}
}

// Necessary?
/* 
#[derive(Debug, Clone, Copy)]
struct Line {
	slope: f32,
	y_intercept: f32
}

impl Line {
	fn intersection(&self, other: Line) -> f32 {
		// https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_line_equations
		// ptx = (d - c) / (a - b)
		(other.y_intercept - self.y_intercept) /
			(self.slope - other.slope)
	}
}

// Slope and Y-intercept
impl From<(f32, f32)> for Line {
	fn from(v: (f32, f32)) -> Line {
		Line {
			slope: v.0,
			y_intercept: v.1
		}
	}
}

// X and Y of 2 points on the line
impl From<(Vec2, Vec2)> for Line {
	fn from(v: (Vec2, Vec2)) -> Line {
		let slope = (v.1.y - v.0.y) / (v.1.x - v.0.x);
		let y_intercept = slope * v.0.x.min(v.1.x);
		Line {
			slope,
			y_intercept
		}
	}
}
 */

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn z_at_flat() -> Result<(), ()> {
		let positions: Vec<Vec2> = vec![
			Vec2::new(16., 16.),
			Vec2::new(-16., 16.),
			Vec2::new(-16., -16.),
			Vec2::new(16., -16.),
		];
		let flat_height = 16.;
		let flat_plane = Plane::Flat(flat_height);

		positions.iter().for_each(|&pos| {
			assert_eq!(flat_height, flat_plane.z_at(pos))
		});
		Ok(())
	}

	#[test]
	fn z_at_sloped() -> Result<(), ()> {
		let positions: Vec<Vec2> = vec![
			Vec2::new(16., 16.),
			Vec2::new(-16., 16.),
			Vec2::new(-16., -16.),
			Vec2::new(16., -16.),
		];

		let sloped_plane = {
			// Based on y = 0.5x where x = 1
			// Here's an interactive graph on Desmos:
			// https://www.desmos.com/calculator/ippcw92fwc
			let x: f32 = 1.0;
			let y: f32 = 0.5 * x;
			let l = (x * x + y * y).sqrt();
			// Normal vectors are perpendicular to lines/planes.
			let a = -y / l;
			let c = x / l;
			Plane::Sloped(a, 0., c, 0.)
		};
		let sloped_heights = vec![8., -8., -8., 8.];

		sloped_heights.iter().zip(positions.iter()).for_each(|(&expected, &pos)| {
			let actual = sloped_plane.z_at(pos);
			// Compare strings because of the inaccuracies caused by how
			// computers represent decimal numbers internally
			let expected = format!("{:.3}", expected);
			let actual = format!("{:.3}", actual);
			assert_eq!(expected, actual);
		});
		Ok(())
	}

	#[test]
	fn z_at_advanced_slope() -> Result<(), ()> {
		// Sloped plane with a more "advanced" slope
		let positions: Vec<Vec2> = vec![
			Vec2::new(16., 16.),
			Vec2::new(-16., 16.),
			Vec2::new(-16., -16.),
			Vec2::new(16., -16.),
		];

		let sloped_plane = {
			// Based on z = .5x + .5y + 5
			let z: f32 = 1.;
			let y = 0.5 * z;
			let x = 0.5 * z;
			let l = (x * x + y * y + z * z).sqrt();
			let a = -x / l;
			let b = -y / l;
			let c = z / l;
			Plane::Sloped(a, b, c, 5. * -c)
		};
		let sloped_heights: Vec<f32> = vec![21., 5., -11., 5.];

		sloped_heights.iter().zip(positions.iter()).for_each(|(&expected, &pos)| {
			let actual = sloped_plane.z_at(pos);
			// Compare strings because of the inaccuracies caused by how
			// computers represent decimal numbers internally
			let expected = format!("{:.3}", expected);
			let actual = format!("{:.3}", actual);
			assert_eq!(expected, actual);
		});
		Ok(())
	}

	#[test]
	fn plane_from_triangle() -> Result<(), String> {
		// Coordinates copied from above example
		let x1: f32 = 16.;
		let y1: f32 = 16.;
		let z1: f32 = 21.;
		let x2: f32 = -16.;
		let y2: f32 = 16.;
		let z2: f32 = 5.;
		let x3: f32 = -16.;
		let y3: f32 = -16.;
		let z3: f32 = -11.;
		let expected = {
			let z: f32 = 1.;
			let y = 0.5 * z;
			let x = 0.5 * z;
			let l = (x * x + y * y + z * z).sqrt();
			let a = -x / l;
			let b = -y / l;
			let c = z / l;
			let d = 5. * -c;
			format!("{:.3} {:.3} {:.3} {:.3}", a, b, c, d)
		};
		let plane = Plane::from_triangle(
			Vec3::new(x1, y1, z1),
			Vec3::new(x2, y2, z2),
			Vec3::new(x3, y3, z3)
		);
		if let Plane::Sloped(a, b, c, d) = plane {
			let actual = format!("{:.3} {:.3} {:.3} {:.3}", a, b, c, d);
			assert_eq!(expected, actual);
			Ok(())
		} else {
			Err(String::from("Plane is not sloped!"))
		}
	}

	#[test]
	fn intersection() {
		let pa = Plane::from_triangle(
			Vec3::from_array([8., 7., 1.]),
			Vec3::from_array([6., 7., 1.]),
			Vec3::from_array([8., 9., 2.]));
		let pb = Plane::from_triangle(
			Vec3::from_array([8., 7., 3.]),
			Vec3::from_array([10., 7., 3.]),
			Vec3::from_array([8., 9., 1.]));
		let intersection_point = pa.intersection(Vec2::from_array([8., 9.]), Vec2::from_array([8., 7.]), &pb);
		assert!(intersection_point.is_some());

		let expected = Vec3::from_array([8.0, 8.333333333, 1.666666666]);
		let intersection_point = intersection_point.map(|v| format!("{:.3} {:.3} {:.3}", v.x, v.y, v.z)).unwrap();
		let expected = format!("{:.3} {:.3} {:.3}", expected.x, expected.y, expected.z);
		assert_eq!(expected, intersection_point);

		// Same as above, with a shorter line
		let no_intersection = pa.intersection(Vec2::from_array([8., 7.5]), Vec2::from_array([8., 7.]), &pb);
		assert_eq!(no_intersection, None);
	}
}


//! Sector floor/ceiling planes
use crate::vector::Vector2;
/// The geometric plane of a sector floor/ceiling
#[derive(Debug, Clone, PartialEq)]
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

impl Plane {
	/// Get the height of the sector plane at the given position
	/// 
	/// # Examples
	/// 
	/// ```
	/// use map_to_3D::plane::Plane;
	/// use map_to_3D::vector::Vector2;
	/// 
	/// let flat_plane = Plane::Flat(16.);
	/// let pos = Vector2::new(16., 16.);
	/// assert_eq!(flat_plane.z_at(&pos), 16.);
	/// ```
	pub fn z_at(&self, pos: &Vector2) -> f32 {
		match self {
			Plane::Flat(height) => *height,
			Plane::Sloped(
				a, b, c, d
			) => {
				// https://github.com/Talon1024/jsdoom/blob/5c3ca7553b/src/convert/3DMapBuilder.ts#L650
				// Also https://github.com/coelckers/gzdoom/blob/7ba5a74f2e/src/gamedata/r_defs.h#L356
				let x = pos.x();
				let y = pos.y();
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
	/// 
	/// let expected: [f32; 3] = [0.0f32, 0.0f32, 1.0f32];
	/// let actual = Plane::Flat(5.).normal(false);
	/// assert_eq!(expected, actual);
	/// ```
	/// 
	/// With a slope defined by the equation `z = .5x + .5y`:
	/// ```
	/// use map_to_3D::plane::Plane;
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
	/// 	16., 16., 21.,
	/// 	-16., 16., 5.,
	/// 	-16., -16., -11.
	/// ).normal(false);
	/// let actual: String = actual.iter().map(|co| format!("{:.3} ", co)).collect();
	/// assert_eq!(expected, actual);
	/// ```
	pub fn normal(&self, reverse: bool) -> [f32; 3] {
		match self {
			Plane::Flat(_) => [0., 0., if reverse {-1.} else {1.}],
			Plane::Sloped(a, b, c, _) => {
				if reverse {
					// I don't know if this is correct
					[-*a, -*b, -*c]
				} else {
					[*a, *b, *c]
				}
			}
		}
	}

	/// Create a `SectorPlane` from three points (a triangle)
	/// 
	/// # Examples
	/// 
	/// With a flat plane:
	/// ```
	/// use map_to_3D::plane::Plane;
	/// 
	/// let expected = Plane::Flat(5.);
	/// let actual = Plane::from_triangle(16., 16., 5., -16., 16., 5., -16., -16., 5.);
	/// assert_eq!(expected, actual);
	/// ```
	/// 
	/// With a slope defined by the equation `z = .5x + .5y`:
	/// ```
	/// use map_to_3D::plane::Plane;
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
	/// 	16., 16., 21.,
	/// 	-16., 16., 5.,
	/// 	-16., -16., -11.
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
		x1: f32,
		y1: f32,
		z1: f32,
		x2: f32,
		y2: f32,
		z2: f32,
		x3: f32,
		y3: f32,
		z3: f32,
	) -> Plane {
		if z1 == z2 && z1 == z3 {
			Plane::Flat(z1)
		} else {
			// Diff point 1 and 2
			let d1x = x2 - x1;
			let d1y = y2 - y1;
			let d1z = z2 - z1;
			// Diff point 1 and 3
			let d2x = x3 - x1;
			let d2y = y3 - y1;
			let d2z = z3 - z1;
			// Calculate ABC
			let a = d1y * d2z - d1z * d2y; // a2b3 - a3b2
			let b = d1z * d2x - d1x * d2z; // a3b1 - a1b3
			let c = d1x * d2y - d1y * d2x; // a1b2 - a2b1
			let l = (a * a + b * b + c * c).sqrt();
			let a = a / l;
			let b = b / l;
			let c = c / l;
			// Calculate D
			// Ax + By + Cz + D = 0
			// Ax + By + Cz = -D
			// D = -(Ax + By + Cz)
			let d = -(a * x1 + b * y1 + c * z1);
			Plane::Sloped(a, b, c, d)
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn z_at_flat() -> Result<(), ()> {
		let positions: Vec<Vector2> = vec![
			Vector2::new(16., 16.),
			Vector2::new(-16., 16.),
			Vector2::new(-16., -16.),
			Vector2::new(16., -16.),
		];
		let flat_height = 16.;
		let flat_plane = Plane::Flat(flat_height);

		positions.iter().for_each(|pos| {
			assert_eq!(flat_height, flat_plane.z_at(pos))
		});
		Ok(())
	}

	#[test]
	fn z_at_sloped() -> Result<(), ()> {
		let positions: Vec<Vector2> = vec![
			Vector2::new(16., 16.),
			Vector2::new(-16., 16.),
			Vector2::new(-16., -16.),
			Vector2::new(16., -16.),
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

		sloped_heights.iter().zip(positions.iter()).for_each(|(&expected, pos)| {
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
		let positions: Vec<Vector2> = vec![
			Vector2::new(16., 16.),
			Vector2::new(-16., 16.),
			Vector2::new(-16., -16.),
			Vector2::new(16., -16.),
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
		let sloped_heights = vec![21., 5., -11., 5.];

		sloped_heights.iter().zip(positions.iter()).for_each(|(&expected, pos)| {
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
			x1, y1, z1,
			x2, y2, z2,
			x3, y3, z3
		);
		if let Plane::Sloped(a, b, c, d) = plane {
			let actual = format!("{:.3} {:.3} {:.3} {:.3}", a, b, c, d);
			assert_eq!(expected, actual);
			Ok(())
		} else {
			Err(String::from("Plane is not sloped!"))
		}
	}
}


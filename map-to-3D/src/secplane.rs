use crate::vector::Vector2;

/// The geometric plane of a sector floor/ceiling
#[derive(Debug, Clone)]
pub enum SectorPlane {
	/// A flat sector plane, represented by a single floating point value,
	/// which is the height of the plane
	Flat(f32),
	/// A sloped sector plane, represented internally by the terms of a plane
	/// equation (`Ax + By + Cz + D = 0`). A, B, and C are the XYZ components
	/// of a normal vector.
	Sloped {
		a: f32,
		b: f32,
		c: f32,
		d: f32
	}
}

impl SectorPlane {
	/// Get the height of the sector plane at the given position
	/// 
	/// # Examples
	/// 
	/// ```
	/// use map_to_3D::secplane::SectorPlane;
	/// use map_to_3D::vector::Vector2;
	/// 
	/// let flat_plane = SectorPlane::Flat(16.);
	/// let pos = Vector2::new(16., 16.);
	/// assert_eq!(flat_plane.z_at(&pos), 16.);
	/// ```
	pub fn z_at(&self, pos: &Vector2) -> f32 {
		match self {
			SectorPlane::Flat(height) => *height,
			SectorPlane::Sloped {
				a, b, c, d
			} => {
				// https://github.com/Talon1024/jsdoom/blob/5c3ca7553b6579e26546781d1746d29819f5a784/src/convert/3DMapBuilder.ts#L650
				// Also https://github.com/coelckers/gzdoom/blob/7ba5a74f2e7fb02f6ed6a9d6a545a4c4ce0a330c/src/gamedata/r_defs.h#L356
				let x = pos.x();
				let y = pos.y();
				let dividend = a * x + b * y + d;
				dividend / -c
			}
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
		let flat_plane = SectorPlane::Flat(flat_height);

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

		let slope = {
			// Based on y = 0.5x where x = 1
			// Here's an interactive graph on Desmos:
			// https://www.desmos.com/calculator/ippcw92fwc
			let x: f32 = 1.0;
			let y: f32 = 0.5 * x;
			let l = (x * x + y * y).sqrt();
			// Normal vectors are perpendicular to lines/planes.
			(-y / l, x / l)
		};
		let sloped_heights = vec![8., -8., -8., 8.];
		let sloped_plane = SectorPlane::Sloped {
			a: slope.0, b: 0., c: slope.1, d: 0.};

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
			SectorPlane::Sloped {a, b, c, d: 5. * -c}
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
}


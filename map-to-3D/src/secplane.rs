use crate::vector::Vector2;

#[derive(Debug, Clone)]
enum SectorPlane {
	Flat(f32),
	// First number is the height, the other four make up the normal vector
	// and distance from the "origin".
	Sloped {
		a: f32,
		b: f32,
		c: f32,
		d: f32
	}
}

impl SectorPlane {
	fn height_at(&self, pos: &Vector2) -> f32 {
		match self {
			SectorPlane::Flat(height) => *height,
			SectorPlane::Sloped {
				a, b, c, d
			} => {
				// https://github.com/Talon1024/jsdoom/blob/5c3ca7553b6579e26546781d1746d29819f5a784/src/convert/3DMapBuilder.ts#L650
				let negative_c = -c;
				let dividend = a * pos.x() + b * pos.y() + d;
				dividend / negative_c
			}
		}
	}
}

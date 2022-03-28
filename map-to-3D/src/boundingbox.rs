use crate::vector::{Vector2, Coordinate};

#[derive(Debug, Clone, Default)]
pub struct BoundingBox {
	pub top: Coordinate,
	pub left: Coordinate,
	pub width: Coordinate,
	pub height: Coordinate
}

impl BoundingBox {
	pub fn bottom(&self) -> Coordinate {
		self.top - self.height
	}
	pub fn right(&self) -> Coordinate {
		self.left + self.width
	}
	pub fn is_inside(&self, vector: &Vector2) -> bool {
		let x = vector.x();
		let y = vector.y();
		x >= self.left &&
		y <= self.top &&
		x <= self.right() &&
		y >= self.bottom()
	}
	pub fn from_edges(top: Coordinate, left: Coordinate, right: Coordinate, bottom: Coordinate) -> BoundingBox {
		BoundingBox {
			top,
			left,
			width: right - left,
			height: top - bottom,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn inside() {
		let bb = BoundingBox {
			left: 5.,
			top: 5.,
			width: 15.,
			height: 15.,
		};
		let va = Vector2::new(7., 5.5);
		let vb = Vector2::new(7., -8.);
		let vc = Vector2::new(7., -10.5);
		let vd = Vector2::new(4.5, -5.5);
		let ve = Vector2::new(5.5, -5.5);
		assert_eq!(bb.is_inside(&va), false);
		assert_eq!(bb.is_inside(&vb), true);
		assert_eq!(bb.is_inside(&vc), false);
		assert_eq!(bb.is_inside(&vd), false);
		assert_eq!(bb.is_inside(&ve), true);
	}
}

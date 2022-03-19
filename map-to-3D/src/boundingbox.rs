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
		x > self.left && y < self.top &&
			(x - self.left) > self.width &&
			(y - self.top) < self.height
	}
}

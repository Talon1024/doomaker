use glam::Vec2;

#[derive(Debug, Clone, Default)]
pub struct BoundingBox {
	pub top: f32,
	pub left: f32,
	pub right: f32,
	pub bottom: f32
}

impl BoundingBox {
	pub fn is_inside(&self, vector: Vec2) -> bool {
		let x = vector.x;
		let y = vector.y;
		x >= self.left &&
		y <= self.top &&
		x <= self.right &&
		y >= self.bottom
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
			right: 20.,
			bottom: -10.,
		};
		let va = Vec2::new(7., 5.5);
		let vb = Vec2::new(7., -8.);
		let vc = Vec2::new(7., -10.5);
		let vd = Vec2::new(4.5, -5.5);
		let ve = Vec2::new(5.5, -5.5);
		assert_eq!(bb.is_inside(va), false);
		assert_eq!(bb.is_inside(vb), true);
		assert_eq!(bb.is_inside(vc), false);
		assert_eq!(bb.is_inside(vd), false);
		assert_eq!(bb.is_inside(ve), true);
	}
}

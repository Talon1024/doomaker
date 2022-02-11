use std::ops::{Add,Sub,AddAssign,SubAssign,Mul,MulAssign,Div,DivAssign};

// #[derive(Hash, PartialEq, Eq, Clone, Debug)]
#[derive(PartialEq, PartialOrd, Clone, Debug, Copy, Default)]
pub struct Vector2(f32, f32);

impl Vector2 {
	pub fn dot(&self, other: &Vector2) -> f32 {
		self.0 * other.0 + self.1 * other.1
	}
	pub fn cross(&self, other: &Vector2) -> f32 {
		self.0 * other.1 - self.1 * other.0
	}
	pub fn angle(&self) -> f32 {
		f32::atan2(self.1, self.0)
	}
	pub fn x(&self) -> f32 {
		self.0
	}
	pub fn y(&self) -> f32 {
		self.1
	}
	pub fn length(&self) -> f32 {
		self.dot(self).sqrt()
	}
	pub fn new(x: f32, y: f32) -> Vector2 {
		Vector2(x, y)
	}
}

impl Add<Vector2> for Vector2 {
	type Output = Vector2;
	fn add(self, other: Vector2) -> Vector2 {
		Vector2(self.0 + other.0, self.1 + other.1)
	}
}

impl Add<&Vector2> for Vector2 {
	type Output = Vector2;
	fn add(self, other: &Vector2) -> Vector2 {
		Vector2(self.0 + other.0, self.1 + other.1)
	}
}

impl Add<&Vector2> for &Vector2 {
	type Output = Vector2;
	fn add(self, other: &Vector2) -> Vector2 {
		Vector2(self.0 + other.0, self.1 + other.1)
	}
}

impl Add<f32> for Vector2 {
	type Output = Vector2;
	fn add(self, other: f32) -> Vector2 {
		Vector2(self.0 + other, self.1 + other)
	}
}

impl Sub<Vector2> for Vector2 {
	type Output = Vector2;
	fn sub(self, other: Vector2) -> Vector2 {
		Vector2(self.0 - other.0, self.1 - other.1)
	}
}

impl Sub<&Vector2> for Vector2 {
	type Output = Vector2;
	fn sub(self, other: &Vector2) -> Vector2 {
		Vector2(self.0 - other.0, self.1 - other.1)
	}
}

impl Sub<&Vector2> for &Vector2 {
	type Output = Vector2;
	fn sub(self, other: &Vector2) -> Vector2 {
		Vector2(self.0 - other.0, self.1 - other.1)
	}
}

impl Sub<f32> for Vector2 {
	type Output = Vector2;
	fn sub(self, other: f32) -> Vector2 {
		Vector2(self.0 - other, self.1 - other)
	}
}

impl Mul<Vector2> for Vector2 {
	type Output = Vector2;
	fn mul(self, other: Vector2) -> Vector2 {
		Vector2(self.0 * other.0, self.1 * other.1)
	}
}

impl Mul<&Vector2> for Vector2 {
	type Output = Vector2;
	fn mul(self, other: &Vector2) -> Vector2 {
		Vector2(self.0 * other.0, self.1 * other.1)
	}
}

impl Mul<&Vector2> for &Vector2 {
	type Output = Vector2;
	fn mul(self, other: &Vector2) -> Vector2 {
		Vector2(self.0 * other.0, self.1 * other.1)
	}
}

impl Mul<f32> for Vector2 {
	type Output = Vector2;
	fn mul(self, other: f32) -> Vector2 {
		Vector2(self.0 * other, self.1 * other)
	}
}

impl Div<Vector2> for Vector2 {
	type Output = Vector2;
	fn div(self, other: Vector2) -> Vector2 {
		Vector2(self.0 / other.0, self.1 / other.1)
	}
}

impl Div<&Vector2> for Vector2 {
	type Output = Vector2;
	fn div(self, other: &Vector2) -> Vector2 {
		Vector2(self.0 / other.0, self.1 / other.1)
	}
}

impl Div<&Vector2> for &Vector2 {
	type Output = Vector2;
	fn div(self, other: &Vector2) -> Vector2 {
		Vector2(self.0 / other.0, self.1 / other.1)
	}
}

impl Div<f32> for Vector2 {
	type Output = Vector2;
	fn div(self, other: f32) -> Vector2 {
		Vector2(self.0 / other, self.1 / other)
	}
}

impl AddAssign<Vector2> for Vector2 {
	fn add_assign(&mut self, other: Vector2) {
		self.0 += other.0;
		self.1 += other.1;
	}
}

impl AddAssign<&Vector2> for Vector2 {
	fn add_assign(&mut self, other: &Vector2) {
		self.0 += other.0;
		self.1 += other.1;
	}
}

impl AddAssign<f32> for Vector2 {
	fn add_assign(&mut self, other: f32) {
		self.0 += other;
		self.1 += other;
	}
}

impl SubAssign<Vector2> for Vector2 {
	fn sub_assign(&mut self, other: Vector2) {
		self.0 -= other.0;
		self.1 -= other.1;
	}
}

impl SubAssign<&Vector2> for Vector2 {
	fn sub_assign(&mut self, other: &Vector2) {
		self.0 -= other.0;
		self.1 -= other.1;
	}
}

impl SubAssign<f32> for Vector2 {
	fn sub_assign(&mut self, other: f32) {
		self.0 -= other;
		self.1 -= other;
	}
}

impl MulAssign<Vector2> for Vector2 {
	fn mul_assign(&mut self, other: Vector2) {
		self.0 *= other.0;
		self.1 *= other.1;
	}
}

impl MulAssign<&Vector2> for Vector2 {
	fn mul_assign(&mut self, other: &Vector2) {
		self.0 *= other.0;
		self.1 *= other.1;
	}
}

impl MulAssign<f32> for Vector2 {
	fn mul_assign(&mut self, other: f32) {
		self.0 *= other;
		self.1 *= other;
	}
}

impl DivAssign<Vector2> for Vector2 {
	fn div_assign(&mut self, other: Vector2) {
		self.0 /= other.0;
		self.1 /= other.1;
	}
}

impl DivAssign<&Vector2> for Vector2 {
	fn div_assign(&mut self, other: &Vector2) {
		self.0 /= other.0;
		self.1 /= other.1;
	}
}

impl DivAssign<f32> for Vector2 {
	fn div_assign(&mut self, other: f32) {
		self.0 /= other;
		self.1 /= other;
	}
}

impl From<&[f32]> for Vector2 {
	fn from(v: &[f32]) -> Vector2 {
		Vector2(
			v.get(0).cloned().unwrap_or(0.),
			v.get(1).cloned().unwrap_or(0.),
		)
	}
}

impl From<(f32, f32)> for Vector2 {
	fn from(v: (f32, f32)) -> Vector2 {
		Vector2(v.0, v.1)
	}
}

impl Eq for Vector2 {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn dot_product() {
		let a = Vector2::from((1.0, 0.0));
		let b = Vector2::from((0.0, 1.0));
		let c = Vector2::from((-1.0, 0.0));
		let d = Vector2::from((0.0, -1.0));
		assert_eq!(a.dot(&a), 1.0);
		assert_eq!(a.dot(&b), 0.0);
		assert_eq!(a.dot(&c), -1.0);
		assert_eq!(a.dot(&d), 0.0);

		assert_eq!(b.dot(&a), 0.0);
		assert_eq!(b.dot(&b), 1.0);
		assert_eq!(b.dot(&c), 0.0);
		assert_eq!(b.dot(&d), -1.0);
	}

	#[test]
	fn cross_product() {
		let a = Vector2::from((1.0, 0.0));
		let b = Vector2::from((0.0, 1.0));
		let c = Vector2::from((-1.0, 0.0));
		let d = Vector2::from((0.0, -1.0));
		assert_eq!(a.cross(&a), 0.0);
		assert_eq!(a.cross(&b), 1.0);
		assert_eq!(a.cross(&c), 0.0);
		assert_eq!(a.cross(&d), -1.0);

		assert_eq!(b.cross(&a), -1.0);
		assert_eq!(b.cross(&b), 0.0);
		assert_eq!(b.cross(&c), 1.0);
		assert_eq!(b.cross(&d), 0.0);
	}

	#[test]
	fn angle() {
		let a = Vector2::from((1.0, 0.0));
		let b = Vector2::from((0.0, 1.0));
		let c = Vector2::from((-1.0, 0.0));
		let d = Vector2::from((0.0, -1.0));
		assert_eq!(a.angle(), 0.0);
		assert_eq!(b.angle(), std::f32::consts::FRAC_PI_2);
		assert_eq!(c.angle(), std::f32::consts::PI);
		assert_eq!(d.angle(), -std::f32::consts::FRAC_PI_2);
	}
}

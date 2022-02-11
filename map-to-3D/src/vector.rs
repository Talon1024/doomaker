//! # 2D vectors
//! 
//! A two-dimensional vector used to represent vertex positions, lines, and
//! possibly other things.

use std::ops::{Add,Sub,AddAssign,SubAssign,Mul,MulAssign,Div,DivAssign};

// #[derive(Hash, PartialEq, Eq, Clone, Debug)]
#[derive(PartialEq, PartialOrd, Clone, Debug, Copy, Default)]
pub struct Vector2(f32, f32);

impl Vector2 {
	/// The dot product of this vector and another
	/// 
	/// # Example:
	/// 
	/// ```
	/// use map_to_3D::vector::Vector2;
	/// let a = Vector2::new(2.0, 2.0);
	/// let b = Vector2::new(1.5, 1.0);
	/// assert_eq!(a.dot(&b), 5.)
	/// ```
	pub fn dot(&self, other: &Vector2) -> f32 {
		self.0 * other.0 + self.1 * other.1
	}
	/// The "cross product" of this vector and another. The cross product
	/// is not defined for 2D vectors, so it is based on this:
	/// <http://allenchou.net/2013/07/cross-product-of-2d-vectors/>
	/// 
	/// # Example:
	/// 
	/// ```
	/// use map_to_3D::vector::Vector2;
	/// let a = Vector2::new(2.0, 2.0);
	/// let b = Vector2::new(1.5, 1.0);
	/// assert_eq!(a.cross(&b), -1.);
	/// ```
	pub fn cross(&self, other: &Vector2) -> f32 {
		self.0 * other.1 - self.1 * other.0
	}
	/// Get the angle between this vector and the origin, assuming the origin
	/// is (0, 0).
	/// 
	/// # Example:
	/// 
	/// ```
	/// use map_to_3D::vector::Vector2;
	/// let a = Vector2::new(2.0, 2.0);
	/// assert_eq!(a.angle().to_degrees().round(), 45.0);
	/// ```
	pub fn angle(&self) -> f32 {
		#[allow(unused_imports)]
		use micromath::F32Ext;
		self.1.atan2(self.0)
	}
	/// Get the X coordinate of this vector.
	/// 
	/// # Example:
	/// 
	/// ```
	/// use map_to_3D::vector::Vector2;
	/// let a = Vector2::new(2.0, 1.0);
	/// assert_eq!(a.x(), 2.0);
	/// ```
	pub fn x(&self) -> f32 {
		self.0
	}
	/// Get the Y coordinate of this vector.
	/// 
	/// # Example:
	/// 
	/// ```
	/// use map_to_3D::vector::Vector2;
	/// let a = Vector2::new(2.0, 1.0);
	/// assert_eq!(a.y(), 1.0);
	/// ```
	pub fn y(&self) -> f32 {
		self.1
	}
	/// Get the length of this vector, or the distance from the origin,
	/// assuming the origin is (0.0, 0.0)
	/// 
	/// # Example:
	/// 
	/// ```
	/// use map_to_3D::vector::Vector2;
	/// let a = Vector2::new(3.0, 4.0);
	/// assert_eq!(a.length(), 5.0);
	/// ```
	pub fn length(&self) -> f32 {
		self.dot(self).sqrt()
	}
	/// Make a new Vector2
	/// 
	/// # Example:
	/// 
	/// ```
	/// use map_to_3D::vector::Vector2;
	/// let a = Vector2::new(2.0, 1.0);
	/// assert_eq!(a, Vector2::new(2.0, 1.0));
	/// ```
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

	#[test]
	fn length() {
		let a = Vector2::from((0.5, 0.5));
		assert_eq!(a.length(), 0.707106781);
	}
}

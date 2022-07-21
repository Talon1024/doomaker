use glam::{EulerRot, f32::{Vec2, Vec3, Mat4, Quat}};
use glow::{Context, HasContext, NativeUniformLocation};
use crate::renderer::UniformDataSource;
use std::f32::consts::{PI, TAU, FRAC_PI_2};

#[derive(Debug, Clone, Copy)]
pub enum FieldOfView {
	Horizontal(f32),
	Vertical(f32),
}

impl Default for FieldOfView {
	fn default() -> Self {
		FieldOfView::Horizontal(FRAC_PI_2)
	}
}

impl FieldOfView {
	pub fn fov_y(&self, asra: f32) -> f32 {
		match self {
			FieldOfView::Horizontal(r) => {
				*r / asra
			},
			FieldOfView::Vertical(r) => {
				*r
			}
		}
	}
}

#[derive(Debug, Clone, Default)]
pub struct Camera {
	pub pos: Vec3,
	pub ori: Vec2,
	pub fov: FieldOfView,
	pub asra: f32,
	pub near: f32,
	pub far: f32,
	pub uniloc: Option<NativeUniformLocation>,
}

const MOUSE_FACTOR: f32 = 0.00390625;

impl UniformDataSource for Camera {
	fn set_uniforms(&self, glc: &Context) {
		unsafe {
		glc.uniform_matrix_4_f32_slice(
			self.uniloc.as_ref(), false,
			&self.projection_view().to_cols_array());
		}
	}

	fn set_textures(&self, _: &Context) {}
}

impl Camera {
	pub fn fov_y(&self) -> f32 {
		self.fov.fov_y(self.asra)
	}

	pub fn projection_view(&self) -> Mat4 {
		self.projection() * self.view()
	}

	pub fn projection(&self) -> Mat4 {
		Mat4::perspective_rh_gl(self.fov_y(), self.asra, self.near, self.far)
	}

	/// Quaternion for view transformation/orientation
	pub fn ori_quat(&self) -> Quat {
		// Original working code
		// Quat::from_rotation_x(-self.ori.y) *
		// Quat::from_rotation_z(-self.ori.x - FRAC_PI_2)
		Quat::from_euler(EulerRot::XYZ, 
				-self.ori.y, 0., 
				-self.ori.x - FRAC_PI_2)
	}

	/// Quaternion for movement vector rotation
	pub fn vrot_quat(&self) -> Quat {
		Quat::from_euler(EulerRot::ZYX,
			self.ori.x + PI + FRAC_PI_2, 0., FRAC_PI_2 - self.ori.y)
	}

	pub fn view(&self) -> Mat4 {
		Mat4::from_quat(self.ori_quat()) * Mat4::from_translation(self.pos)
	}

	pub fn rotate(&mut self, x: f32, y: f32) {
		self.ori.x -= (x * MOUSE_FACTOR) as f32;
		self.ori.x = self.ori.x.rem_euclid(TAU);
		self.ori.y -= (y * MOUSE_FACTOR) as f32;
		self.ori.y = self.ori.y.clamp(0., PI);
	}

	pub fn direction(&self) -> Vec3 {
		let (th, ph) = (self.ori.x, self.ori.y);
		let (tc, ts, pc, ps) = (th.cos(), th.sin(), ph.cos(), ph.sin());
		Vec3::new(
			tc * ps,
			ts * ps,
			pc
		)
	}
}

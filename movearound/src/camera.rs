use glam::{EulerRot,f32::{Vec2, Vec3, Mat4, Quat}};
use glow::{Context, HasContext, NativeUniformLocation};
use crate::renderer::UniformDataSource;
use std::f32::consts::{PI, TAU};

#[derive(Debug, Clone, Default)]
pub struct Camera {
	pub pos: Vec3,
	pub ori: Vec2,
	pub fovy: f32,
	pub asra: f32,
	pub near: f32,
	pub far: f32,
	pub uniloc: Option<NativeUniformLocation>,
}

const MOUSE_FACTOR: f32 = 0.001953125;

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
	pub fn projection_view(&self) -> Mat4 {
		self.projection() * self.view()
	}

	pub fn projection(&self) -> Mat4 {
		Mat4::perspective_lh(self.fovy, self.asra, self.near, self.far)
	}

	pub fn ori_quat(&self) -> Quat {
		Quat::from_euler(EulerRot::XYZ,
			self.ori.y, 0., self.ori.x)
	}

	pub fn view(&self) -> Mat4 {
		Mat4::from_quat(self.ori_quat()) * Mat4::from_translation(self.pos)
	}

	pub fn rotate(&mut self, x: f32, y: f32) {
		self.ori.x -= (x * MOUSE_FACTOR) as f32;
		self.ori.x = self.ori.x.rem_euclid(TAU);
		self.ori.y -= (y * MOUSE_FACTOR) as f32;
		self.ori.y = self.ori.y.clamp(-PI, 0.);
	}
}

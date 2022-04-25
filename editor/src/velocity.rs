use glam::Vec3;

pub struct Velocity {
	vel: Vec3,
	max: f32,
}

impl Velocity {
	pub fn new(max: f32, initial: Option<Vec3>) -> Velocity {
		Velocity {
			vel: initial.unwrap_or(Vec3::ZERO),
			max
		}
	}
	pub fn move_dir(&mut self, dir: Vec3, accel: f32) {
		let dest = dir * self.max;
		let diff = (dest - self.vel).clamp_length(0., accel);
		self.vel += diff;
	}
	pub fn get(&self) -> Vec3 {
		self.vel
	}
	pub fn speed(&self) -> f32 {
		self.vel.length()
	}
}

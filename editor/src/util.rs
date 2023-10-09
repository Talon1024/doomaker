use macroquad::prelude::*;
use macroquad::miniquad as nq;

pub mod math {
    use super::*;
    pub fn vec3_from_orientation(o: (f32, f32)) -> Vec3 {
        vec3_from_spherical(o.0, o.1)
    }
    pub fn vec3_from_spherical(ph: f32, th: f32) -> Vec3 {
        Vec3::from((
            ph.cos() * th.sin(),
            ph.sin() * th.sin(),
            th.cos(),
        ))
    }
}

pub fn fov_x_to_y(x: f32) -> f32 {
    let aspect_ratio = screen_width() / screen_height();
    x / aspect_ratio
}

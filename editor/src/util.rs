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

pub mod gl {
    use super::*;
    pub fn to_texture(image: Image) -> Texture2D {
        let mut context = unsafe { get_internal_gl().quad_context };
        let texture = nq::Texture::from_data_and_format(
            &mut context, &image.bytes[..], nq::TextureParams {
            format: nq::TextureFormat::RGBA8,
            wrap: nq::TextureWrap::Repeat,
            filter: nq::FilterMode::Linear,
            width: image.width as u32,
            height: image.height as u32,
        });
        Texture2D::from_miniquad_texture(texture)
    }
}

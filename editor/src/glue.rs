use map_to_3D::vector::Vector2;
use map_to_3D::secplane::SectorPlane;
use macroquad::prelude::*;
use macroquad::models as mq;

pub fn sector_vertices(
	verts: &[Vector2],
	plane: &SectorPlane,
	colour: Option<Color>
) -> Vec<mq::Vertex> {
	verts.iter().map(|v| {
		mq::Vertex {
			position: Vec3::new(v.x(), v.y(), plane.z_at(&v)),
			uv: Vec2::new(v.x() / 64., v.y() / 64.),
			color: colour.unwrap_or(WHITE),
		}
	}).collect()
}

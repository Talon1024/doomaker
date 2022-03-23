use map_to_3D::vertex::MapVertex;
use map_to_3D::secplane::SectorPlane;
use macroquad::prelude::*;
use macroquad::models as mq;

pub fn sector_vertices(
	verts: &[MapVertex],
	plane: &SectorPlane,
	colour: Option<Color>
) -> Vec<mq::Vertex> {
	verts.iter().map(|v| {
		mq::Vertex {
			position: Vec3::new(v.p.x(), v.p.y(), plane.z_at(&v.p)),
			uv: Vec2::new(v.p.x() / 64., v.p.y() / 64.),
			color: colour.unwrap_or(WHITE),
		}
	}).collect()
}

use macroquad::prelude::*;
use map_to_3D::edge::Edge;
use map_to_3D::plane::Plane;
use map_to_3D::sectorpolygonbuilder as spb;
use macroquad::models as mq;

/*
pub fn tiny_texture() -> Texture2D {
	Texture2D::from_rgba8(3, 3, &[
		/*
		// regx: #([0-9A-Fa-f]{2})([0-9A-Fa-f]{2})([0-9A-Fa-f]{2})([0-9A-Fa-f]{2})
		// repl: 0x$1, 0x$2, 0x$3, 0x$4,
		#FF0000FF #FFFF00FF #00FF00FF
		#FF00FFFF #00000000 #000000FF
		#0000FFFF #00FFFFFF #FFFFFFFF
		*/
		0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF,
		0xFF, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF,
		0x00, 0x00, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
	])
}

pub fn cube_mesh() -> Box<Mesh> {
	use macroquad::models::Vertex;
	Box::new(Mesh {
		/*
		1------0
		|      |
		|      |
		|      |
		2------3
		*/
		vertices: vec![
			Vertex { position: const_vec3!([1., 1., 1.]), uv: Vec2::X, color: WHITE },
			Vertex { position: const_vec3!([-1., 1., 1.]), uv: Vec2::ZERO, color: WHITE },
			Vertex { position: const_vec3!([-1., -1., 1.]), uv: Vec2::Y, color: WHITE },
			Vertex { position: const_vec3!([1., -1., 1.]), uv: Vec2::ONE, color: WHITE },

			Vertex { position: const_vec3!([1., 1., -1.]), uv: Vec2::X, color: WHITE },
			Vertex { position: const_vec3!([-1., 1., -1.]), uv: Vec2::ZERO, color: WHITE },
			Vertex { position: const_vec3!([-1., -1., -1.]), uv: Vec2::Y, color: WHITE },
			Vertex { position: const_vec3!([1., -1., -1.]), uv: Vec2::ONE, color: WHITE }
		],
		indices: vec![
			/*
			// regx: quad!\((\d+),\s*(\d+),\s*(\d+),\s*(\d+)\)(,?)
			// repl: $1, $2, $3, $1, $3, $4$5
			quad!(0, 1, 2, 3),
			quad!(4, 5, 6, 7),
			quad!(0, 1, 5, 4),
			quad!(1, 2, 6, 5),
			quad!(2, 3, 7, 6),
			quad!(3, 0, 4, 7)
			*/
			0, 1, 2, 0, 2, 3,
			4, 5, 6, 4, 6, 7,
			0, 1, 5, 0, 5, 4,
			1, 2, 6, 1, 6, 5,
			2, 3, 7, 2, 7, 6,
			3, 0, 4, 3, 4, 7
		],
		texture: Some(tiny_texture())
	})
}
*/

/*
fn rainbow(h: f32) -> Color {
	use std::f32::consts::PI;
	let positions: [f32; 3] = [0., 0.333333333333, 0.666666666666];
	let rgb: Vec<f32> = positions.iter().map(|p| {
		let co = f32::cos(h - PI * 2. * p) + 0.5;
		co.clamp(0., 1.)
	}).collect();
	Color::new(rgb[0], rgb[1], rgb[2], 1.)
}
fn rainbowi(h: usize) -> Color {
	use std::f32::consts::PI;
	let h = (h as f32) / (180. / PI);
	rainbow(h)
}
*/

fn sector_vertices(
	verts: &[Vec2],
	plane: &Plane,
	colour: Option<Color>
) -> Vec<mq::Vertex> {
	verts.iter().map(|&v| {
		mq::Vertex {
			position: Vec3::new(v.x, v.y, plane.z_at(v)),
			uv: Vec2::new(v.x / 64., v.y / 64.),
			color: colour.unwrap_or(WHITE),
		}
	}).collect()
}

pub fn holey_mesh() -> Box<Mesh> {
	let verts: Vec<Vec2> = vec![
		Vec2::new(70., 30.),	// 0
		Vec2::new(68., 30.),
		Vec2::new(69., 32.),
		Vec2::new(64., 64.),	// 3
		Vec2::new(64., -64.),
		Vec2::new(-64., -64.),
		Vec2::new(-64., 64.),
		Vec2::new(44., 52.),	// 7
		Vec2::new(-52., 52.),
		Vec2::new(-52., -44.),
		Vec2::new(52., 44.),	// 10
		Vec2::new(52., -52.),
		Vec2::new(-44., -52.),	// 12
	];
	let edges = vec![
		Edge::new(3, 4),	// 0
		Edge::new(4, 5),
		Edge::new(5, 6),
		Edge::new(6, 3),
		Edge::new(7, 8),	// 4
		Edge::new(8, 9),
		Edge::new(9, 7),
		Edge::new(10, 11),
		Edge::new(11, 12),	// 8
		Edge::new(12, 10),
		Edge::new(0, 1),
		Edge::new(1, 2),
		Edge::new(2, 0),	// 12
	];
	let sp = Plane::Flat(-7.);
	let polys = spb::build_polygons(&edges, &verts);
	let ptris = spb::auto_triangulate(&polys, &verts);
	// let mut hue = 0;
	Box::new(Mesh {
		vertices: sector_vertices(&verts, &sp, None),
		indices: ptris.iter().filter_map(|p| {
			if let Some(p) = p {
				Some(p)
			} else {
				None
			}
		}).flat_map(|p| p.iter().map(|&i| i as u16)).collect(),
		texture: None
	})
}

use macroquad::prelude::*;

pub fn tiny_texture() -> Texture2D {
	Texture2D::from_rgba8(3, 3, &[
		255, 0, 0, 255,     255, 255, 0, 255,   0, 255, 0, 255,
		255, 0, 255, 255,   0, 0, 0, 0,         255, 255, 0, 255,
		0, 0, 255, 255,     255, 0, 255, 255,   255, 255, 255, 255,
	])
}

pub fn cube_mesh() -> Mesh {
	use macroquad::models::Vertex;
	Mesh {
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
			// regx: (^\s*)square!\((\d+),\s*(\d+),\s*(\d+),\s*(\d+)\)(,?)
			// repl: $1$2, $3, $4, $2, $4, $5$6
			square!(0, 1, 2, 3),
			square!(4, 5, 6, 7),
			square!(0, 1, 5, 4),
			square!(1, 2, 6, 5),
			square!(2, 3, 7, 6),
			square!(3, 0, 4, 7)
			*/
			0, 1, 2, 0, 2, 3,
			4, 5, 6, 4, 6, 7,
			0, 1, 5, 0, 5, 4,
			1, 2, 6, 1, 6, 5,
			2, 3, 7, 2, 7, 6,
			3, 0, 4, 3, 4, 7
		],
		texture: Some(tiny_texture())
	}
}

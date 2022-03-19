use macroquad::prelude::*;

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
	}
}

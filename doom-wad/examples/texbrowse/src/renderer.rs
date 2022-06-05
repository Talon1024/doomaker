use glam::Vec3;
use glow::{Context, HasContext};
use std::{
	ops::Range,
	path::Path,
	fs::File,
	io::Read,
	error::Error,
	mem,
	slice,
};

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Vertex3D {
	pub position: Vec3,
	pub colour: Vec3,
	pub normal: Vec3,
	pub fog_colour: Vec3,
	pub fog_dist: f32,
}
impl Vertex3D {
	pub const ATTR_POSITION: u32 = 0;
	pub const ATTR_COLOUR: u32 = 1;
	pub const ATTR_NORMAL: u32 = 2;
	pub const ATTR_FOG_COLOUR: u32 = 3;
	pub const ATTR_FOG_DIST: u32 = 4;
}

#[derive(Debug, Clone, Default)]
pub struct Data3D {
	pub vertices: Vec<Vertex3D>,
	pub indices: Option<Vec<u32>>,
	pub program: Option<glow::NativeProgram>,
	pub vertex_buffer: Option<glow::NativeBuffer>,
	pub index_buffer: Option<glow::NativeBuffer>,
	pub vertex_array: Option<glow::NativeVertexArray>,
}

pub trait Renderable {
	fn setup(&mut self, glc: &Context);
	fn update(&mut self, glc: &Context) {}
	fn draw(&self, glc: &Context);
}

pub unsafe fn ptr_range_to_u8_slice<'a, T>(range: Range<*const T>) -> &'a [u8] {
	let start = range.start as *const u8;
	let end = range.end as *const u8;
	let len = end.offset_from(start) as usize;
	slice::from_raw_parts(start, len)
}

pub fn init_shaders(
	vertex_shader_source_file: &(impl AsRef<Path> + ?Sized),
	fragment_shader_source_file: &(impl AsRef<Path> + ?Sized),
	glc: &Context
) -> Result<glow::NativeProgram, Box<dyn Error>> {
	let prog = unsafe { glc.create_program() }?;
	let vertex_shader = unsafe { glc.create_shader(glow::VERTEX_SHADER) }?;
	let fragment_shader = unsafe { glc.create_shader(glow::FRAGMENT_SHADER) }?;
	let cleanup = || {
		unsafe {
			glc.delete_program(prog);
			glc.delete_shader(vertex_shader);
			glc.delete_shader(fragment_shader);
		}
	};
	let vertex_source = {
		let mut f = File::open(vertex_shader_source_file.as_ref())?;
		let mut text = String::new();
		f.read_to_string(&mut text)?;
		text
	};
	let fragment_source = {
		let mut f = File::open(fragment_shader_source_file.as_ref())?;
		let mut text = String::new();
		f.read_to_string(&mut text)?;
		text
	};
	unsafe {
		glc.shader_source(vertex_shader, &vertex_source);
		glc.shader_source(fragment_shader, &fragment_source);
	}
	unsafe {
		glc.compile_shader(vertex_shader);
	}
	if !unsafe { glc.get_shader_compile_status(vertex_shader) } {
		let rv = Err(format!("Vertex shader error:\n{}", unsafe { glc.get_shader_info_log(vertex_shader) }));
		cleanup();
		rv?;
	} else {
		println!("{}", unsafe { glc.get_shader_info_log(vertex_shader) });
	}
	unsafe { glc.compile_shader(fragment_shader); }
	if !unsafe { glc.get_shader_compile_status(fragment_shader) } {
		let rv = Err(format!("Fragment shader error:\n{}", unsafe { glc.get_shader_info_log(fragment_shader) }));
		cleanup();
		rv?;
	} else {
		println!("{}", unsafe { glc.get_shader_info_log(fragment_shader) });
	}
	unsafe {
		glc.attach_shader(prog, vertex_shader);
		glc.attach_shader(prog, fragment_shader);
		glc.link_program(prog);
	}
	if !unsafe { glc.get_program_link_status(prog) } {
		let rv = Err(format!("Program link error:\n{}", unsafe { glc.get_program_info_log(prog) }));
		cleanup();
		rv?;
	} else {
		println!("{}", unsafe { glc.get_program_info_log(prog) });
	}
	Ok(prog)
}

impl Renderable for Data3D {
	fn setup(&mut self, glc: &Context) {
		let vec_size = mem::size_of::<Vec3>() as i32;
		let stride = mem::size_of::<Vertex3D>() as i32;
		self.vertex_buffer = Some(unsafe { glc.create_buffer() }.expect("Cannot create vbuffer"));
		if let Some(_) = &self.indices {
			self.index_buffer = Some(unsafe { glc.create_buffer() }.expect("Cannot create ibuffer"));
		}
		self.vertex_array = Some(unsafe { glc.create_vertex_array() }.expect("Cannot create array"));
		unsafe {
			// STEP: Bind and upload index buffer
			if let Some(b) = &self.indices {
			let raw_index_data = ptr_range_to_u8_slice(b.as_ptr_range());
			glc.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, self.index_buffer);
			glc.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, raw_index_data, glow::STATIC_DRAW);
			glc.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
			}

			// STEP: Set up vertex array (attributes)
			glc.bind_vertex_array(self.vertex_array);

			// STEP: Bind and upload vertex buffer
			let raw_vertex_data = ptr_range_to_u8_slice(self.vertices.as_ptr_range());
			glc.bind_buffer(glow::ARRAY_BUFFER, self.vertex_buffer);
			glc.buffer_data_u8_slice(glow::ARRAY_BUFFER, raw_vertex_data, glow::STATIC_DRAW);

			glc.vertex_attrib_pointer_f32(
				Vertex3D::ATTR_POSITION, 3, glow::FLOAT, false,
				stride, 0);
			glc.enable_vertex_attrib_array(Vertex3D::ATTR_POSITION);

			glc.vertex_attrib_pointer_f32(
				Vertex3D::ATTR_COLOUR, 3, glow::FLOAT, false,
				stride, vec_size);
			glc.enable_vertex_attrib_array(Vertex3D::ATTR_COLOUR);

			glc.vertex_attrib_pointer_f32(
				Vertex3D::ATTR_NORMAL, 3, glow::FLOAT, false,
				stride, vec_size * 2);
			glc.enable_vertex_attrib_array(Vertex3D::ATTR_NORMAL);

			glc.vertex_attrib_pointer_f32(
				Vertex3D::ATTR_FOG_COLOUR, 3, glow::FLOAT, false,
				stride, vec_size * 3);
			glc.enable_vertex_attrib_array(Vertex3D::ATTR_FOG_COLOUR);

			glc.vertex_attrib_pointer_f32(
				Vertex3D::ATTR_FOG_DIST, 1, glow::FLOAT, false,
				stride, vec_size * 4);
			glc.enable_vertex_attrib_array(Vertex3D::ATTR_FOG_DIST);

			// STEP: Unbind buffers
			glc.bind_vertex_array(None);
			// When using vertex array objects, this has to be done after
			// setting up the attributes, so that the vertex array stores a
			// reference to the vertex buffer.
			glc.bind_buffer(glow::ARRAY_BUFFER, None);
		}
	}
	fn draw(&self, glc: &Context) {
		unsafe {
			// STEP: Set shader program
			glc.use_program(self.program);
			// STEP: Upload uniforms
			// STEP: Bind vertex buffer and array
			glc.bind_vertex_array(self.vertex_array);
			// STEP: Draw call
			match &self.indices {
				Some(indices) => {
					glc.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, self.index_buffer);
					glc.draw_elements(glow::TRIANGLES, indices.len() as i32, glow::UNSIGNED_INT, 0);
				},
				None => {
					// glc.bind_buffer(glow::ARRAY_BUFFER, self.vertex_buffer);
					glc.draw_arrays(glow::TRIANGLES, 0, self.vertices.len() as i32);
				}
			}
			/* glc.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
			glc.bind_buffer(glow::ARRAY_BUFFER, None);
			glc.bind_vertex_array(None);
			glc.use_program(None); */
		}
	}
}

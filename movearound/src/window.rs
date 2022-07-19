use std::error::Error;
use glutin::{
	event_loop::EventLoop,
	ContextBuilder,
	ContextWrapper,
	PossiblyCurrent,
	GlRequest::GlThenGles,
	GlProfile,
	window::{Window, WindowBuilder},
	dpi::Size::Logical,
};
use glow::Context;

pub fn init_window(size: Option<[f32; 2]>) -> Result<(Context, ContextWrapper<PossiblyCurrent, Window>, EventLoop<()>), Box<dyn Error>> {
	let win = WindowBuilder::new()
		.with_inner_size(Logical(size.unwrap_or([550.0f32, 300.]).into()))
		.with_title("App-o")
		.with_visible(true);
	let el = EventLoop::new();
	let ctx = ContextBuilder::new()
		.with_gl(GlThenGles {
			opengl_version: (3, 3),
			opengles_version: (2, 0),
		})
		.with_gl_profile(GlProfile::Core)
		.with_depth_buffer(16)
		.with_double_buffer(Some(true))
		.build_windowed(win, &el)?;
	// OpenGL/glutin context
	let ctx = unsafe { ctx.make_current().map_err(|(_, e)| {
		Box::<dyn Error>::from(e)
	}) }?;
	// glow context
	let glc = unsafe { Context::from_loader_function(|n| ctx.get_proc_address(n))};
	Ok((glc, ctx, el))
}

use std::{
	error::Error,
	rc::Rc,
	fs::File,
	f32::consts::FRAC_PI_2
};
use egui_glow::EguiGlow;
use glutin::event_loop::ControlFlow;
use glow::HasContext;
use glam::f32::{Vec2, Vec3};
use debugs::quat_debug_window;

mod window;
mod renderer;
mod camera;
mod util;
mod debugs;

use renderer::{Data3D, Vertex3D, Renderable};

fn main() -> Result<(), Box<dyn Error>> {
	// STEP: Set up window and context
	let (wid, hei) = (720, 500);
	let (glc, ctx, el) = window::init_window(Some([wid as f32, hei as f32]))?;
	let glc = Rc::from(glc);
	let mut egui_glow = EguiGlow::new(ctx.window(), Rc::clone(&glc));
	let _user_event = el.create_proxy();

	// STEP: Set up things to render
	let mut my_cube = Data3D::new(
		vec![
			/* 0--3
			   |\ |
			   | \|
			   1--2 */
			//+Z
			Vertex3D {position: Vec3::new(-10., 10., 10.), uv: Vec2::new(0.6666666666, 0.), ..Default::default()},
			Vertex3D {position: Vec3::new(-10., -10., 10.), uv: Vec2::new(0.6666666666, 0.5), ..Default::default()},
			Vertex3D {position: Vec3::new(10., -10., 10.), uv: Vec2::new(1., 0.5), ..Default::default()},
			Vertex3D {position: Vec3::new(10., 10., 10.), uv: Vec2::new(1., 0.), ..Default::default()},

			//-Z
			Vertex3D {position: Vec3::new(-10., 10., -10.), uv: Vec2::new(0.6666666666, 0.5), ..Default::default()},
			Vertex3D {position: Vec3::new(-10., -10., -10.), uv: Vec2::new(0.6666666666, 1.), ..Default::default()},
			Vertex3D {position: Vec3::new(10., -10., -10.), uv: Vec2::new(1., 1.), ..Default::default()},
			Vertex3D {position: Vec3::new(10., 10., -10.), uv: Vec2::new(1., 0.5), ..Default::default()},

			//+Y
			Vertex3D {position: Vec3::new(10., 10., 10.), uv: Vec2::new(0.3333333333, 0.), ..Default::default()},
			Vertex3D {position: Vec3::new(10., 10., -10.), uv: Vec2::new(0.3333333333, 0.5), ..Default::default()},
			Vertex3D {position: Vec3::new(-10., 10., -10.), uv: Vec2::new(0.6666666666, 0.5), ..Default::default()},
			Vertex3D {position: Vec3::new(-10., 10., 10.), uv: Vec2::new(0.6666666666, 0.), ..Default::default()},

			//-Y
			Vertex3D {position: Vec3::new(-10., -10., 10.), uv: Vec2::new(0.3333333333, 0.5), ..Default::default()},
			Vertex3D {position: Vec3::new(-10., -10., -10.), uv: Vec2::new(0.3333333333, 1.), ..Default::default()},
			Vertex3D {position: Vec3::new(10., -10., -10.), uv: Vec2::new(0.6666666666, 1.), ..Default::default()},
			Vertex3D {position: Vec3::new(10., -10., 10.), uv: Vec2::new(0.6666666666, 0.5), ..Default::default()},

			//+X
			Vertex3D {position: Vec3::new(10., -10., 10.), uv: Vec2::new(0., 0.), ..Default::default()},
			Vertex3D {position: Vec3::new(10., -10., -10.), uv: Vec2::new(0., 0.5), ..Default::default()},
			Vertex3D {position: Vec3::new(10., 10., -10.), uv: Vec2::new(0.3333333333, 0.5), ..Default::default()},
			Vertex3D {position: Vec3::new(10., 10., 10.), uv: Vec2::new(0.3333333333, 0.), ..Default::default()},

			//-X
			Vertex3D {position: Vec3::new(-10., 10., 10.), uv: Vec2::new(0., 0.5), ..Default::default()},
			Vertex3D {position: Vec3::new(-10., 10., -10.), uv: Vec2::new(0., 1.), ..Default::default()},
			Vertex3D {position: Vec3::new(-10., -10., -10.), uv: Vec2::new(0.3333333333, 1.), ..Default::default()},
			Vertex3D {position: Vec3::new(-10., -10., 10.), uv: Vec2::new(0.3333333333, 0.5), ..Default::default()},
		],
		Some(vec![
			0, 1, 2, 0, 2, 3,
			4, 5, 6, 4, 6, 7,
			8, 9, 10, 8, 10, 11,
			12, 13, 14, 12, 14, 15,
			16, 17, 18, 16, 18, 19,
			20, 21, 22, 20, 22, 23]),
		{
			let mut file = File::open("cube256.png")?;
			renderer::texture_png(&glc, &mut file).ok()
		},
		Some(renderer::init_shaders(
			"simple.vert", "simple.frag", &glc)?)
		//..Default::default()
	);
	my_cube.setup(&glc)?;
	let mut pointer_lock = false;
	let asra = wid as f32 / hei as f32;
	let mut camera = camera::Camera {
		fovy: util::fov_x_to_y(100., asra).to_radians(),
		asra,
		near: 0.0625,
		far: 10000.0,
		ori: Vec2::new(0., FRAC_PI_2),
		uniloc: my_cube.program.and_then(|prog| {
			unsafe { glc.get_uniform_location(prog, "u_projview") }
		}),
		..Default::default()
	};
	// STEP: Initial OpenGL calls
	unsafe {
		glc.viewport(0, 0, wid, hei);
		glc.clear_buffer_f32_slice(glow::DEPTH, 0, &[0.0f32]);
		glc.clear_buffer_f32_slice(glow::COLOR, 0, &[0.0f32]);
		glc.enable(glow::DEPTH_TEST);
		glc.enable(glow::CULL_FACE);
		glc.cull_face(glow::BACK);
		glc.front_face(glow::CW);
	}
	// STEP: Load WAD and textures
	// TODO: TextureBrowser struct
	el.run(move |event, _, control_flow| {
		match event {
			glutin::event::Event::WindowEvent { window_id: _window_id, event } => {
				// println!("WindowEvent window_id {:?} event {:?}", window_id, event);
				use glutin::event::WindowEvent::*;
				use glutin::event::{ElementState, MouseButton, VirtualKeyCode as VKC};
				if !egui_glow.on_event(&event) {
					match event {
						Resized(size) => {
							ctx.resize(size);
							let asra = size.width as f32 / size.height as f32;
							camera.asra = asra;
							camera.fovy = util::fov_x_to_y(100., asra).to_radians();
							unsafe{glc.viewport(0, 0, size.width as i32, size.height as i32);}
						},
						CloseRequested => {
							*control_flow = ControlFlow::Exit;
						},
						KeyboardInput { device_id: _, input, is_synthetic: _ } => {
							if pointer_lock {
								match input.virtual_keycode {
									Some(VKC::Escape) => {
										pointer_lock = false;
										// ctx.window() must be used because otherwise
										// "borrowed value does not live long enough"
										ctx.window().set_cursor_visible(true);
									},
									Some(VKC::W) => {
										let direction =
											camera.ori_quat().mul_vec3(Vec3::new(0., 1., 0.));
										camera.pos += direction;
									},
									Some(VKC::S) => {
										let direction =
											camera.ori_quat().mul_vec3(Vec3::new(0., -1., 0.));
										camera.pos += direction;
									},
									Some(VKC::A) => {
										let direction =
											camera.ori_quat().mul_vec3(Vec3::new(-1., 0., 0.));
										camera.pos += direction;
									},
									Some(VKC::D) => {
										let direction =
											camera.ori_quat().mul_vec3(Vec3::new(1., 0., 0.));
										camera.pos += direction;
									},
									Some(VKC::Q) => {
										let direction =
											camera.ori_quat().mul_vec3(Vec3::new(0., 0., 1.));
										camera.pos += direction;
									},
									Some(VKC::Z) => {
										let direction =
											camera.ori_quat().mul_vec3(Vec3::new(0., 0., -1.));
										camera.pos += direction;
									},
									_ => (),
								}
							}
						},
						MouseInput { state, button, .. } => {
							match (state, button) {
								(ElementState::Pressed, MouseButton::Left) => {
									if !pointer_lock {
										pointer_lock = true;
										ctx.window().set_cursor_visible(false);
									}
								},
								_ => ()
							}
						},
						_ => (),
					}
				}
			},
			glutin::event::Event::DeviceEvent { device_id:_, event } => {
				use glutin::event::DeviceEvent;
				match event {
					DeviceEvent::MouseMotion { delta: (x, y) } => {
						if pointer_lock {
							use glutin::dpi::{LogicalPosition, LogicalSize};

							let winscale = ctx.window().scale_factor();
							let winsize: LogicalSize<f64> = ctx.window().inner_size()
								.to_logical(winscale);
							let winsize = [winsize.width / 2., winsize.height / 2.];
							if let Err(e) = ctx.window().set_cursor_position(LogicalPosition::<f64>::from(winsize)) {
								eprintln!("{:?}", e);
							}
							camera.rotate(x as f32, y as f32);
						}
					},
					_ => ()
				}
			},
			glutin::event::Event::UserEvent(e) => {
				println!("user event {:?}", e);
			},/* 
			glutin::event::Event::Suspended => {
				println!("suspended");
			},
			glutin::event::Event::Resumed => {
				println!("resumed");
			}, */
			glutin::event::Event::MainEventsCleared => {
				unsafe {
					glc.clear_color(0.125, 0.125, 0.125, 1.0);
					glc.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
				}
				my_cube.draw(&glc, &camera);
				my_cube.update(&glc);
				egui_glow.run(ctx.window(), |ectx| {
					egui::TopBottomPanel::top("main menu").show(ectx, |ui| {
						egui::menu::bar(ui, |ui| {
							egui::menu::menu_button(ui, "File", |ui| {
								if ui.button("Open..").clicked() {}
								if ui.button("Exit").clicked() {
									*control_flow = ControlFlow::Exit;
								}
							});
						});
					});
					quat_debug_window(&camera, ectx);
					// matrix_debug_window(&camera, ectx, &mut debug_window_mtx_choice);
				});
				egui_glow.paint(ctx.window());
				if let Err(e) = ctx.swap_buffers() {
					eprintln!("Swap buffer error: {:?}", e);
				}
			},
			_ => (),
			/* 
			glutin::event::Event::RedrawRequested(id) => {
				// println!("redraw requested on window {:?}", id);
			},
			glutin::event::Event::RedrawEventsCleared => {
				()
			},
			glutin::event::Event::LoopDestroyed => {
				println!("Loop destroyed");
			}, */
		}
	});
}


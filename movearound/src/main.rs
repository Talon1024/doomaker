use std::{
	error::Error,
	sync::Arc,
	fs::{File, OpenOptions},
	f32::consts::FRAC_PI_2
};
use egui_glow::EguiGlow;
use glutin::{
	event_loop::ControlFlow,
	event::VirtualKeyCode as VKC
};
use glow::HasContext;
use glam::f32::{Vec2, Vec3};
use serde::{Serialize, Deserialize};

mod window;
mod renderer;
mod camera;
mod debugs;
mod input;

use input::*;
use renderer::{Data3D, Vertex3D, Renderable};

#[derive(Debug, Clone, Default)]
pub(crate) struct App {
	pub camera: camera::Camera,
	pub mode: Mode,
	pub preferences: Preferences,
}

static CONFIG_FILENAME: &str = "config.yml";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct Preferences {
	pub keybinds: KeyboardBindings,
}

impl Preferences {
	fn load() -> Result<Self, Box<dyn Error>> {
		let file = File::open(CONFIG_FILENAME)?;
		serde_yaml::from_reader(file).map_err(|e| e.into())
	}
	fn save(&self) -> Result<(), Box<dyn Error>> {
		let file = OpenOptions::new().write(true).create(true)
		.open(CONFIG_FILENAME)?;
		serde_yaml::to_writer(file, self).map_err(|e| e.into())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	// STEP: Set up window and context
	let (wid, hei) = (720, 500);
	let (glc, ctx, el) = window::init_window(Some([wid as f32, hei as f32]))?;
	let glc = Arc::from(glc);
	let mut egui_glow = EguiGlow::new(&el, Arc::clone(&glc));
	let user_event = el.create_proxy();

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
			Vertex3D {position: Vec3::new(10., 10., -10.), uv: Vec2::new(0.6666666666, 0.5), ..Default::default()},
			Vertex3D {position: Vec3::new(10., -10., -10.), uv: Vec2::new(0.6666666666, 1.), ..Default::default()},
			Vertex3D {position: Vec3::new(-10., -10., -10.), uv: Vec2::new(1., 1.), ..Default::default()},
			Vertex3D {position: Vec3::new(-10., 10., -10.), uv: Vec2::new(1., 0.5), ..Default::default()},

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
			0, 1, 2, 0, 2, 3, // Z+
			4, 5, 6, 4, 6, 7, // Z-
			8, 9, 10, 8, 10, 11, // Y+
			12, 13, 14, 12, 14, 15, // Y-
			16, 17, 18, 16, 18, 19, // X+
			20, 21, 22, 20, 22, 23]), // X-
		{
			let mut file = File::open("cube256.png")?;
			renderer::texture_png(&glc, &mut file).ok()
		},
		Some(renderer::init_shaders(
			"simple.vert", "simple.frag", &glc)?)
		//..Default::default()
	);
	my_cube.setup(&glc)?;
	let asra = wid as f32 / hei as f32;

	let mut app = Box::new(App {
		mode: Mode::View3D,
		camera: camera::Camera {
			fov: camera::FieldOfView::Horizontal(120f32.to_radians()),
			asra,
			near: 0.125,
			far: 10000.0,
			ori: Vec2::new(0., FRAC_PI_2),
			uniloc: my_cube.program.and_then(|prog| {
				unsafe { glc.get_uniform_location(prog, "u_projview") }
			}),
			..Default::default()
		},
		preferences: Preferences::load()
		.unwrap_or_else(|e| {
			eprintln!("Could not load preferences for some reason:\n\
			{:?}\n\
			Using hard-coded defaults...", e);
			let mut preferences = Preferences::default();
			preferences.keybinds.insert(VKC::W, ActionId::MoveForward);
			preferences.keybinds.insert(VKC::S, ActionId::MoveBackward);
			preferences.keybinds.insert(VKC::Escape, ActionId::ReleasePointer);
			preferences
		}),
		..Default::default()
	});
	// STEP: Initial OpenGL calls
	unsafe {
		glc.viewport(0, 0, wid, hei);
		glc.enable(glow::DEPTH_TEST);
		glc.depth_func(glow::LEQUAL);
		glc.depth_mask(true);
		glc.enable(glow::CULL_FACE);
		glc.cull_face(glow::BACK);
		glc.front_face(glow::CW);
		// glc.clear_buffer_f32_slice(glow::DEPTH, 0, &[0.0f32]);
		// glc.clear_buffer_f32_slice(glow::COLOR, 0, &[0.0f32]);
	}
	// STEP: Load WAD and textures
	// TODO: TextureBrowser struct
	el.run(move |event, _, control_flow| {
		match event {
			glutin::event::Event::WindowEvent { window_id: _window_id, event } => {
				// println!("WindowEvent window_id {:?} event {:?}", window_id, event);
				use glutin::event::WindowEvent::*;
				use glutin::event::{ElementState, MouseButton};
				if !egui_glow.on_event(&event) {
					match event {
						Resized(size) => {
							ctx.resize(size);
							let asra = size.width as f32 / size.height as f32;
							app.camera.asra = asra;
							unsafe{glc.viewport(0, 0, size.width as i32, size.height as i32);}
						},
						CloseRequested => {
							*control_flow = ControlFlow::Exit;
						},
						KeyboardInput { device_id: _, input, is_synthetic: _ } => {
							use glutin::event::ElementState::*;
							let state = match input.state {
								Pressed => input::ActionState::Active,
								Released => input::ActionState::Inactive,
							};
							let action_id = input.virtual_keycode.and_then(
								|vkc| app.preferences.keybinds.get(&vkc));
							if let Some(id) = action_id {
								if let Err(e) = user_event.send_event(Action {
									id: *id,
									state,
								}) {
									eprintln!("{:?}", e);
								}
							}
						},
						MouseInput { state, button, .. } => {
							match (state, button, app.mode) {
								(ElementState::Pressed, MouseButton::Left, Mode::View3D) => {
									if let Err(e) = user_event.send_event(Action {
										id: ActionId::LockPointer,
										state: ActionState::Active,
									}) {
										eprintln!("{:?}", e);
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
						if let Mode::Look3D = app.mode {
							// Set cursor position
							use glutin::dpi::{LogicalPosition, LogicalSize};
							let winscale = ctx.window().scale_factor();
							let winsize: LogicalSize<f64> = ctx.window().inner_size()
								.to_logical(winscale);
							let winsize = [winsize.width / 2., winsize.height / 2.];
							if let Err(e) = ctx.window().set_cursor_position(LogicalPosition::<f64>::from(winsize)) {
								eprintln!("{:?}", e);
							}
							// Rotate camera based on mouse movement
							app.camera.rotate(x as f32, y as f32);
						}
					},
					_ => ()
				}
			},
			glutin::event::Event::UserEvent(e) => {
				e.perform(&ctx, &mut app);
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
					glc.enable(glow::DEPTH_TEST);
					glc.enable(glow::CULL_FACE);
				}
				my_cube.draw(&glc, &app.camera);
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
				});
				egui_glow.paint(ctx.window());
				if let Err(e) = ctx.swap_buffers() {
					eprintln!("Swap buffer error: {:?}", e);
				}
			},
			glutin::event::Event::LoopDestroyed => {
				if let Err(e) = app.preferences.save() {
					eprintln!("Could not save preferences for some reason:\n\
					{:?}", e)
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


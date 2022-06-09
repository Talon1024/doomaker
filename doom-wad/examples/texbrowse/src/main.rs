use std::{error::Error, rc::Rc, fs::File, io::Read};
use egui_glow::EguiGlow;
use glutin::{
	event_loop::ControlFlow,
};
use glow::HasContext;
use glam::f32::Vec3;
use png::{Decoder, Transformations};

mod window;
mod renderer;
mod camera;
mod custom_widgets;

use renderer::{Data3D, Vertex3D, Renderable};
use custom_widgets::tex::TextureSquare;

type VecResult<T> = Result<Vec<T>, Box<dyn Error>>;

fn main() -> Result<(), Box<dyn Error>> {
	// STEP: Set up window and context
	let (glc, ctx, el) = window::init_window(Some([720., 500.]))?;
	let glc = Rc::from(glc);
	let mut egui_glow = EguiGlow::new(ctx.window(), Rc::clone(&glc));
	let _user_event = el.create_proxy();
	// STEP: Set up things to render
	let mut my_rect = Data3D {
		vertices: vec![
			/* 0--3
			   |\ |
			   | \|
			   1--2 */
			// With index buffer
			/*  */
			Vertex3D {position: Vec3::new(-0.5, 0.5, 0.), colour: Vec3::new(1., 0., 0.), ..Default::default()},
			Vertex3D {position: Vec3::new(-0.5, -0.5, 0.), colour: Vec3::new(0., 1., 0.), ..Default::default()},
			Vertex3D {position: Vec3::new(0.5, -0.5, 0.), colour: Vec3::new(0., 0., 1.), ..Default::default()},
			Vertex3D {position: Vec3::new(0.5, 0.5, 0.), colour: Vec3::new(1., 1., 1.), ..Default::default()},
			/*  */
			// No index buffer
			/* 
			Vertex3D {position: Vec3::new(-0.5, 0.5, 0.), colour: Vec3::new(1., 0., 0.), ..Default::default()},
			Vertex3D {position: Vec3::new(-0.5, -0.5, 0.), colour: Vec3::new(0., 1., 0.), ..Default::default()},
			Vertex3D {position: Vec3::new(0.5, -0.5, 0.), colour: Vec3::new(0., 0., 1.), ..Default::default()},
			Vertex3D {position: Vec3::new(-0.5, 0.5, 0.), colour: Vec3::new(1., 0., 0.), ..Default::default()},
			Vertex3D {position: Vec3::new(0.5, -0.5, 0.), colour: Vec3::new(0., 0., 1.), ..Default::default()},
			Vertex3D {position: Vec3::new(0.5, 0.5, 0.), colour: Vec3::new(1., 1., 1.), ..Default::default()},
			 */
		],
		indices: Some(vec![0, 1, 2, 0, 2, 3]),
		program: Some(renderer::init_shaders("simple.vert", "simple.frag", &glc)?),
		..Default::default()
	};
	my_rect.setup(&glc)?;
	// STEP: Initial OpenGL calls
	unsafe {
		glc.viewport(0, 0, 550, 300);
		glc.clear_buffer_f32_slice(glow::DEPTH, 0, &[0.0f32]);
		glc.clear_buffer_f32_slice(glow::COLOR, 0, &[0.0f32]);
		glc.enable(glow::DEPTH_TEST);
		glc.enable(glow::CULL_FACE);
		glc.cull_face(glow::BACK);
	}
	// STEP: Textures
	let tex_names = ["TALLASS", "WIDEASS", "PIVY3", "TINY"];
	let tex_files = ["tallass.png", "wideass.png", "pivy3.png", "tiny.png"];
	let tex_images = tex_files.iter().map(|fname| {
		let file = File::open(fname)?;
		let mut decoder = Decoder::new(file);
		decoder.set_transformations(Transformations::normalize_to_color8());
		let mut reader = decoder.read_info()?;
		let png::Info {width, height, color_type , bit_depth , ..} = *reader.info();
		let channels = match color_type {
			png::ColorType::Grayscale => 1,
			png::ColorType::Rgb => 3,
			png::ColorType::Indexed => 1,
			png::ColorType::GrayscaleAlpha => 2,
			png::ColorType::Rgba => 4,
		};
		let bytes_per_pixel = match bit_depth {
			png::BitDepth::One => 1,
			png::BitDepth::Two => 1,
			png::BitDepth::Four => 1,
			png::BitDepth::Eight => 1,
			png::BitDepth::Sixteen => 2,
		};
		let data = {
			let mut data = vec![0; reader.output_buffer_size()];
			reader.next_frame(&mut data)?;
			data
		};
		let tex = renderer::texture(&glc, &data, width, height, channels,
			bytes_per_pixel)?;
		let txid = egui_glow.painter.register_native_texture(tex);
		Ok(egui::Image::new(txid, (width as f32, height as f32)))
	}).collect::<VecResult<egui::Image>>()?;
	let mut tex_name_filter = String::new();
	let mut tex_full_path = false;
	el.run(move |event, _window, control_flow| {
		match event {
/* 			glutin::event::Event::NewEvents(e) => {
				// println!("new events {:?}", e);
			},
 */			glutin::event::Event::WindowEvent { window_id: _window_id, event } => {
				// println!("WindowEvent window_id {:?} event {:?}", window_id, event);
				if !egui_glow.on_event(&event) {
				match event {
					glutin::event::WindowEvent::Resized(size) => {
						ctx.resize(size);
						unsafe{glc.viewport(0, 0, size.width as i32, size.height as i32);}
					},
					glutin::event::WindowEvent::Moved(_) => (),
					glutin::event::WindowEvent::CloseRequested => {
						*control_flow = ControlFlow::Exit;
					},
					glutin::event::WindowEvent::Destroyed => (),
					glutin::event::WindowEvent::DroppedFile(_) => (),
					glutin::event::WindowEvent::HoveredFile(_) => (),
					glutin::event::WindowEvent::HoveredFileCancelled => (),
					glutin::event::WindowEvent::ReceivedCharacter(_) => (),
					glutin::event::WindowEvent::Focused(_) => (),
					glutin::event::WindowEvent::KeyboardInput { device_id, input, is_synthetic } => (),
					glutin::event::WindowEvent::ModifiersChanged(_) => (),
					glutin::event::WindowEvent::CursorMoved { device_id, position, modifiers } => (),
					glutin::event::WindowEvent::CursorEntered { device_id } => (),
					glutin::event::WindowEvent::CursorLeft { device_id } => (),
					glutin::event::WindowEvent::MouseWheel { device_id, delta, phase, modifiers } => (),
					glutin::event::WindowEvent::MouseInput { device_id, state, button, modifiers } => (),
					glutin::event::WindowEvent::TouchpadPressure { device_id, pressure, stage } => (),
					glutin::event::WindowEvent::AxisMotion { device_id, axis, value } => (),
					glutin::event::WindowEvent::Touch(_) => (),
					glutin::event::WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size } => (),
					glutin::event::WindowEvent::ThemeChanged(_) => (),
				}
			}
			},
			glutin::event::Event::DeviceEvent { device_id:_, event:_ } => {
				// println!("DeviceEvent device_id {:?} event {:?}", device_id, event);
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
					glc.clear_color(1.0, 0.0, 1.0, 1.0);
					glc.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
				}
				my_rect.draw(&glc, None);
				my_rect.update(&glc);
				egui_glow.run(ctx.window(), |ectx| {
					egui::Window::new("Texture browser")
					.default_width(500.)
					.default_height(300.)
					.vscroll(false)
					.resizable(true)
					.collapsible(false)
					.show(ectx, |ui| {
						egui::SidePanel::left("categories")
						.resizable(true)
						.min_width(60.).show_inside(ui, |ui| {
							egui::ScrollArea::vertical().show(ui, |ui| {
								ui.label("Booba");
								ui.label("Feet");
								ui.label("Manga");
								ui.label("Lorian");
								ui.label("Son of a gun!! Good grief.");
							});
						});

						ui.horizontal(|ui| {
							ui.label("Filter:");
							egui::text_edit::TextEdit
							::singleline(&mut tex_name_filter)
							.desired_width(70.).show(ui);
							ui.label("Selected:");
							ui.label("texture");
							ui.checkbox(&mut tex_full_path, "Full path");
						});
						ui.separator();
						egui::ScrollArea::vertical()
						.always_show_scroll(true).show(ui, |ui| {
							let column_width = 48.;
							ui.horizontal_wrapped(|ui| {
								tex_names.iter().zip(tex_images.iter())
								.cycle().take(100).for_each(|(&name, &tex)| {
									let mut ts = TextureSquare {
										size: column_width,
										selected: false,
										tex
									};
									ts.show(ui);
								});
							});
						});
					});
					/* egui::Window::new("texture info").show(ectx, |ui| {
						ectx.texture_ui(ui);
					}); */
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

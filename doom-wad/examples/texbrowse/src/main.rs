use std::{error::Error, rc::Rc, fs::File, ops::Range};
use egui::Widget;
use egui_glow::EguiGlow;
use glutin::{
	event_loop::ControlFlow,
};
use glow::HasContext;
use glam::f32::Vec3;
use png::{Decoder, Transformations};
use doom_wad::{wad::{Namespaced, DoomWad, DoomWadCollection}, res::{read_texturex, PaletteCollection, ToImage, Image}};

mod window;
mod renderer;
// mod camera;
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
	let wads = DoomWadCollection(vec![DoomWad::load_sync("../../tests/data/3difytest.wad")?]);
	let wad_lumps = wads.lump_map();
	let wad_pal = wads.playpal(Some(&wad_lumps)).map(PaletteCollection::from);
	let wad_patches = wads.namespace("patches").map(|mut ns| {
		ns.0.extend(wads.namespace("sprites").iter());
		ns
	}).ok_or("No patches!")?;
	let wad_textures = wads.textures(Some(&wad_lumps), &wad_patches).ok_or("No PNAMES!")?;
	let textures = wad_textures.tex_map();
	let mut tex_names = Vec::new();
	let tex_images = textures.iter().map(|(&name, &tex)| {
		tex_names.push(name.to_string());
		let mut image = tex.to_image();
		let Image {width, height, ..} = image;
		image.to_rgb(wad_pal.as_ref().map(|pc| {pc.get(0).ok()}).flatten());
		let channels = 4;
		let bytes_per_channel = 1;
		let image_data = image.truecolor.as_ref().unwrap().as_flat_samples();
		let tex = renderer::texture(&glc, image_data.samples, width as u32,
			height as u32, channels, bytes_per_channel)?;
		let txid = egui_glow.painter.register_native_texture(tex);
		Ok(egui::Image::new(txid, (width as f32, height as f32)))
	}).collect::<VecResult<egui::Image>>()?;
	let mut tex_name_filter = String::new();
	let mut tex_full_path = false;
	let mut selected_index = 0;
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
	// STEP: Load WAD and textures
	// TODO: TextureBrowser struct
	el.run(move |event, _window, control_flow| {
		match event {
			glutin::event::Event::WindowEvent { window_id: _window_id, event } => {
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
					egui::TopBottomPanel::top("main menu").show(ectx, |ui| {
						egui::menu::bar(ui, |ui| {
							egui::menu::menu_button(ui, "File", |ui| {
								ui.button("Open..");
								if ui.button("Exit").clicked() {
									*control_flow = ControlFlow::Exit;
								}
							});
						});
					});
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
							ui.collapsing("TEXTUREX", |ui| {
								ui.label("Booba");
								ui.label("Feet");
								ui.label("Manga");
								ui.label("Lorian");
								ui.label("Son of a gun!! Good grief.");
							});
							ui.collapsing("All", |ui| {
								ui.label("Booba");
								ui.label("Feet");
								ui.label("Manga");
								ui.label("Lorian");
								ui.label("Son of a gun!! Good grief.");
							});
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
						egui::ScrollArea::vertical().show(ui, |ui| {
							ui.horizontal_wrapped(|ui| {
								(0..).zip(tex_names.iter()).zip(tex_images.iter())
								.for_each(|((index, name), &tex)| {
									let mut ts = TextureSquare::new(
										None, tex, name, selected_index == index
									);
									if ts.ui(ui).clicked() {
										selected_index = index;
									}
								});
							});
						});
					});
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

fn the_gui(ectx: &egui::Context) {
	
}

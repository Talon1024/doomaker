use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PointerMode {
	Free,
	Locked,
}

mod sample_data;
mod util;
mod velocity; use velocity::Velocity;
mod data;
mod property;

#[macroquad::main("Doomaker")]
async fn main() {
	use std::f32::consts::{PI, FRAC_PI_2};
	set_pc_assets_folder("assets");
	let mouse_fac = 0.001953125; // 1 / 512 or 2^-9
	let move_fac = 0.0625; // 1 / 16 or 2^-4
	let move_max_speed = 2.;
	let shift_move_fac = 0.125; // 1 / 8 or 2^-3
	let shift_move_max_speed = 7.;
	let mut orientation = (FRAC_PI_2, FRAC_PI_2);
	let mut cam3d = Camera3D {
		position: Vec3::from((0.,0.,0.)),
		target: util::math::vec3_from_orientation(orientation),
		up: Vec3::Z,
		fovy: util::fov_x_to_y(100f32.to_radians()),
		aspect: None,
		projection: Projection::Perspective,
		render_target: None,
		viewport: None
	};
	let mut cube_mesh = sample_data::holey_mesh();
	cube_mesh.texture = load_image("sky.png").await.ok().map(util::gl::to_texture);
	let mut egui_wants_pointer = false;
	let mut ptr_mode = PointerMode::Free;
	let mut last_mouse_pos = (0.0f32, 0.0f32);
	let mut movement = Vec3::ZERO;
	let mut velocity = Velocity::new(move_max_speed, None);
	let ptr_lock_tex = load_image("ptrlock2.png").await.ok().map(|img| {
		let mut texture_handle: Option<egui::TextureHandle> = None;
		egui_macroquad::cfg(|egui_ctx| {
			texture_handle = Some(egui_ctx.load_texture(
				"pointer_lock",
				egui::ColorImage::from_rgba_unmultiplied(
					[img.width as usize, img.height as usize], &img.bytes)));
		});
		texture_handle.unwrap()
	}).or_else(||{
		println!("ptrlock2.png not found");
		None
	});
	let mut exit = false;
	loop {
		if exit { break; }

		// STEP: Set up egui
		egui_macroquad::ui(|egui_ctx| {
			egui::TopBottomPanel::top("menu_bar").show(egui_ctx, |ui| {
				egui::menu::bar(ui, |ui| {
					ui.menu_button("File", |ui| {
						if ui.button("Open").clicked() {
							println!("Open");
						}
						if ui.button("Save").clicked() {
							println!("Save");
						}
						if ui.button("Save As").clicked() {
							println!("Save As");
						}
						if ui.button("Exit").clicked() {
							exit = true;
						}
					});
					if let Some(tex) = &ptr_lock_tex {
						if let PointerMode::Locked = ptr_mode {
							ui.image(tex, tex.size_vec2());
						}
					}
				});
			});
			egui_wants_pointer = egui_ctx.wants_pointer_input();
		});

		// STEP: Draw stuff
		clear_background(BEIGE);
		set_camera(&cam3d);
		draw_mesh(&cube_mesh);

		// Handle input
		match ptr_mode {
			PointerMode::Free => {
				if is_mouse_button_pressed(MouseButton::Left) &&
				!egui_wants_pointer {
					ptr_mode = PointerMode::Locked;
					set_cursor_grab(true);
					// egui_macroquad::draw() overrides this.
					// show_mouse(false);
				}
			},
			PointerMode::Locked => {
				// Mouse looking
				if is_key_pressed(KeyCode::Escape) {
					ptr_mode = PointerMode::Free;
					set_cursor_grab(false);
					// egui_macroquad::draw() overrides this.
					// show_mouse(true);
				}
				let mouse_delta = mouse_position();
				// println!("Mouse {} {}", mouse_delta.0, mouse_delta.1);
				let mouse_delta = (
					(mouse_delta.0 - last_mouse_pos.0) * mouse_fac,
					(mouse_delta.1 - last_mouse_pos.1) * mouse_fac,
				);

				// Horizontal orientation - wraps around the Z axis
				orientation.0 = (orientation.0 - mouse_delta.0)
					.rem_euclid(PI * 2.);
				// Vertical orientation - clamped to the range 0-PI. Also, PI
				// cannot be the upper bound since it would break the camera
				// matrix
				orientation.1 = (orientation.1 + mouse_delta.1)
					.clamp(mouse_fac, PI - mouse_fac);

				// Forward, left, right, backward movement
				// TODO: Configurable key mapping
				movement = Vec3::new(
					if is_key_down(KeyCode::Z) {1.} else if is_key_down(KeyCode::Q) {-1.} else {0.},
					if is_key_down(KeyCode::A) {1.} else if is_key_down(KeyCode::D) {-1.} else {0.},
					if is_key_down(KeyCode::W) {1.} else if is_key_down(KeyCode::S) {-1.} else {0.},
				);
			}
		}

		// STEP: Draw egui after everything else so it is on top
		egui_macroquad::draw();

		// STEP: Show/hide mouse cursor
		match ptr_mode {
			PointerMode::Free => { show_mouse(true); },
			PointerMode::Locked => { show_mouse(false); },
		}

		// STEP: Apply changes caused by user inputs
		let dir_vec = util::math::vec3_from_orientation(orientation);
		let dir_quat = // Rotation for movement vector
			Quat::from_rotation_z(orientation.0) *
			Quat::from_rotation_y(orientation.1);
		movement = dir_quat.mul_vec3(movement);
		velocity.move_dir(movement, move_fac);
		cam3d.position += velocity.get();
		cam3d.target = cam3d.position + dir_vec;
		cam3d.fovy = util::fov_x_to_y(100f32.to_radians());
		last_mouse_pos = mouse_position();

		exit |= is_quit_requested();
		next_frame().await
	}
}

use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MousePointerMode {
    Free,
    Locked
}

mod sample_data;
mod util;
mod glue;

#[macroquad::main("Editor/viewer")]
async fn main() {
    use std::f32::consts::{PI, FRAC_PI_2};
    set_pc_assets_folder("assets");
    let mouse_fac = 0.001953125; // 1 / 512 or 2^-9
    let move_fac = 0.25; // 1 / 64 or 2^-6
    let mut orientation = (FRAC_PI_2, FRAC_PI_2);
    let mut cam3d = Camera3D {
        position: Vec3::from((0.,0.,0.)),
        target: util::vec3_from_orientation(orientation),
        up: Vec3::Z,
        fovy: util::fov_x_to_y(100f32.to_radians()),
        aspect: None,
        projection: Projection::Perspective,
        render_target: None,
        viewport: None
    };
    let mut cube_mesh = sample_data::holey_mesh();
    cube_mesh.texture = load_texture("sky.png").await.ok();
    let mut ptr_mode = MousePointerMode::Free;
    let mut last_mouse_pos = (0.0f32, 0.0f32);
    let mut movement = Vec3::ZERO;
    let ptr_lock_tex = load_texture("ptrlock.png").await.ok().or_else(||{
        println!("ptrlock.png not found");
        None
    });
    loop {
        // Draw stuff
        clear_background(BEIGE);
        set_camera(&cam3d);
        draw_mesh(&cube_mesh);

        // Handle input
        match ptr_mode {
            MousePointerMode::Free => {
                if is_mouse_button_pressed(MouseButton::Left) {
                    ptr_mode = MousePointerMode::Locked;
                    set_cursor_grab(true);
                    show_mouse(false);
                }
            },
            MousePointerMode::Locked => {
                if let Some(tex) = ptr_lock_tex {
                    set_default_camera();
                    draw_texture(tex, 0.0, 0.0, WHITE);
                    set_camera(&cam3d);
                }
                // Mouse looking
                if is_key_pressed(KeyCode::Escape) {
                    ptr_mode = MousePointerMode::Free;
                    set_cursor_grab(false);
                    show_mouse(true);
                }
                let mouse_delta = mouse_position();
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
                    if is_key_down(KeyCode::Z) {move_fac} else if is_key_down(KeyCode::Q) {-move_fac} else {0.},
                    if is_key_down(KeyCode::A) {move_fac} else if is_key_down(KeyCode::D) {-move_fac} else {0.},
                    if is_key_down(KeyCode::W) {move_fac} else if is_key_down(KeyCode::S) {-move_fac} else {0.},
                );
            }
        }

        // Apply changes caused by user inputs
        let dir_vec = util::vec3_from_orientation(orientation);
        let dir_quat = // Rotation for movement vector
            Quat::from_rotation_z(orientation.0) *
            Quat::from_rotation_y(orientation.1);
        movement = dir_quat.mul_vec3(movement);
        cam3d.position += Vec3::from(movement);
        cam3d.target = cam3d.position + dir_vec;
        cam3d.fovy = util::fov_x_to_y(100f32.to_radians());
        last_mouse_pos = mouse_position();

        next_frame().await
    }
}

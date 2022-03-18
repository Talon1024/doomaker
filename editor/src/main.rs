use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PointerMode {
    Free,
    Locked
}

mod sample_data;
mod util;

#[macroquad::main("Editor/viewer")]
async fn main() {
    use std::f32::consts::{PI, FRAC_PI_2};
    let mouse_fac = 0.001953125;
    let move_fac = 0.015625;
    let mut orientation = (FRAC_PI_2, mouse_fac);
    let mut camera = Camera3D {
        position: Vec3::from((0.,0.,0.)),
        target: util::vec3_from_orientation(orientation),
        up: Vec3::Z,
        fovy: util::fov_x_to_y(100f32.to_radians()),
        aspect: None,
        projection: Projection::Perspective,
        render_target: None,
        viewport: None
    };
    let cube_mesh = sample_data::cube_mesh();
    let mut ptr_mode = PointerMode::Free;
    let mut last_mouse_pos = (0.0f32, 0.0f32);
    let mut movement = Vec3::ZERO;
    loop {
        // Draw stuff
        clear_background(BEIGE);
        set_camera(&camera);
        draw_mesh(&cube_mesh);

        // Handle input
        match ptr_mode {
            PointerMode::Free => {
                if is_mouse_button_pressed(MouseButton::Left) {
                    ptr_mode = PointerMode::Locked;
                    set_cursor_grab(true);
                    show_mouse(false);
                }
            },
            PointerMode::Locked => {
                // Mouse looking
                if is_key_pressed(KeyCode::Escape) {
                    ptr_mode = PointerMode::Free;
                    set_cursor_grab(false);
                    show_mouse(true);
                }
                let mouse_delta = mouse_position();
                let mouse_delta = (
                    (mouse_delta.0 - last_mouse_pos.0) * mouse_fac,
                    (mouse_delta.1 - last_mouse_pos.1) * mouse_fac,
                );
                // println!("{} {}", mouse_delta.0, mouse_delta.1);
                orientation.0 = (orientation.0 - mouse_delta.0)
                    .rem_euclid(PI * 2.);
                orientation.1 = (orientation.1 + mouse_delta.1)
                    .clamp(mouse_fac, PI - mouse_fac);

                // Forward, left, right, backward movement
                movement = Vec3::new(
                    if is_key_down(KeyCode::A) {-move_fac} else if is_key_down(KeyCode::D) {move_fac} else {0.},
                    if is_key_down(KeyCode::W) {move_fac} else if is_key_down(KeyCode::S) {-move_fac} else {0.},
                    0.,
                );
            }
        }

        // Apply changes caused by user inputs
        let dir_vec = util::vec3_from_orientation(orientation);
        let dir_quat =
            Quat::from_rotation_arc(Vec3::Y, dir_vec);
        movement = dir_quat.mul_vec3(movement);
        camera.position += Vec3::from(movement);
        camera.target = camera.position + dir_vec;
        camera.fovy = util::fov_x_to_y(100f32.to_radians());
        last_mouse_pos = mouse_position();

        next_frame().await
    }
}

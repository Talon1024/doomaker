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
    let mut orientation = (std::f32::consts::PI / 2., 0.001953125);
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
                if is_key_pressed(KeyCode::Escape) {
                    ptr_mode = PointerMode::Free;
                    set_cursor_grab(false);
                    show_mouse(true);
                }
                let mouse_delta = mouse_position();
                let mouse_delta = (
                    (mouse_delta.0 - last_mouse_pos.0) * 0.001953125,
                    (mouse_delta.1 - last_mouse_pos.1) * 0.001953125,
                );
                // println!("{} {}", mouse_delta.0, mouse_delta.1);
                orientation.0 = (orientation.0 - mouse_delta.0)
                    .rem_euclid(std::f32::consts::PI * 2.);
                orientation.1 = (orientation.1 + mouse_delta.1)
                    .clamp(0.001953125, std::f32::consts::PI - 0.001953125);
            }
        }
        let direction = util::vec3_from_orientation(orientation);
        camera.target = direction;
        camera.fovy = util::fov_x_to_y(100f32.to_radians());
        last_mouse_pos = mouse_position();

        next_frame().await
    }
}

use glam::{Vec3, Quat, Mat4};

#[derive(Debug, Clone)]
pub enum Projection {
    Perspective {
        fov_y: f32,
        aspect: f32
    },
    Orthographic {
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
    },
}

#[derive(Debug, Clone)]
pub struct Camera {
    pub position: Vec3,
    pub orientation: Quat,
    pub projection: Projection,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn view_matrix(&self) -> Mat4 {
        let view = Mat4::from_rotation_translation(
            self.orientation, -self.position);
        let projection = match self.projection {
            Projection::Perspective { fov_y, aspect } => {
                Mat4::perspective_rh_gl(fov_y, aspect, self.near, self.far)
            },
            Projection::Orthographic { left: l, right: r, bottom: b, top: t } => {
                Mat4::orthographic_rh_gl(l, r, b, t, self.near, self.far)
            },
        };
        projection * view
    }
}

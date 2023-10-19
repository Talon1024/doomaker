use glam::{Vec2, Vec3};

#[derive(Debug, Clone)]
pub struct LineQuad {
    pub vertices: [Vec3; 4]
}

#[derive(Debug, Clone)]
pub struct LineQuads (Vec<LineQuad>);

impl LineQuads {

}

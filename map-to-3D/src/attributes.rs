use glam::{Vec2, Vec3, Vec4};

pub trait VertexAttributes {
    type IndexType;
    fn position(&self) -> Vec3;
    fn index(&self) -> Box<[Self::IndexType]>;
    fn colour(&self) -> Vec3;
    fn fog(&self) -> Vec4; // `w` component is distance
    fn uv(&self) -> Vec2;
}

use glam::{Vec2, Vec3};

#[derive(Debug, Clone)]
pub(crate) enum PropertyValue {
    String(String),
    Integer(i32),
    UnsignedInteger(u32),
    Index(usize),
    Boolean(bool),
    Float(f32),
    Vec2(Vec2),
    Vec3(Vec3),
}

pub(crate) trait Properties {
    fn set_property(&mut self, prop: &str, value: Option<PropertyValue>);
    fn get_property(&self, prop: &str) -> Option<PropertyValue>;
}

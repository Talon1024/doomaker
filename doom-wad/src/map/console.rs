//! Console port Doom map structures

#[derive(Debug, Clone)]
pub struct Vertex {
    pub x: i32,
    pub y: i32
}

#[derive(Debug, Clone)]
pub struct Linedef64 {
    pub a: u16,
    pub b: u16,
    pub flags: u16,
    pub special: u32,
    pub tag: u16,
    pub front: u16,
    pub back: u16,
}

#[derive(Debug, Clone)]
pub struct Sidedef64 {
    pub x: i16,
    pub y: i16,
    pub upper: u16,
    pub lower: u16,
    pub middle: u16,
    pub sec: u16,
}

#[derive(Debug, Clone)]
pub struct Sector64 {
    /// Floor height
    pub florh: i16,
    /// Ceiling height
    pub ceilh: i16,
    /// Floor material
    pub flort: u16,
    /// Ceiling material
    pub ceilt: u16,
    /// Floor colour index
    pub florc: u16,
    /// Ceiling colour index
    pub ceilc: u16,
    /// Thing colour index
    pub thngc: u16,
    /// Wall top colour index
    pub waltc: u16,
    /// Wall bottom colour index
    pub walbc: u16,
    pub special: i16,
    pub tag: i16,
    pub flags: u16,
}

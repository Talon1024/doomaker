// Structures for the original Doom map format
pub struct Vertex {
    pub x: i16,
    pub y: i16
}

pub struct Linedef {
    pub a: u16,
    pub b: u16,
    pub flags: u16,
    pub special: u16,
    pub tag: u16,
    pub front: u16,
    pub back: u16,
}

pub struct Sidedef {
    pub x: i16,
    pub y: i16,
    pub upper: String,
    pub lower: String,
    pub middle: String,
    pub sec: u16,
}

pub struct Sector {
    /// Floor height
    pub florh: i16,
    /// Ceiling height
    pub ceilh: i16,
    /// Floor material
    pub flort: String,
    /// Ceiling material
    pub ceilt: String,
    pub light: i16,
    pub special: i16,
    pub tag: i16,
}

pub struct Thing {
    pub x: i16,
    pub y: i16,
    pub angle: i16,
    pub ednum: i16,
    pub flags: i16,
}

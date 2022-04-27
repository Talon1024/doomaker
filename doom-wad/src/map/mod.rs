// Structures for the original Doom map format
use crate::wad::{DoomWad, DoomWadLump};
use std::io::{Cursor, Read};
mod lumps;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub x: i16,
    pub y: i16
}

#[derive(Debug, Clone)]
pub struct Linedef {
    pub a: u16,
    pub b: u16,
    pub flags: u16,
    pub special: u16,
    pub tag: u16,
    pub front: u16,
    pub back: u16,
}

#[derive(Debug, Clone)]
pub struct Sidedef {
    pub x: i16,
    pub y: i16,
    pub upper: String,
    pub lower: String,
    pub middle: String,
    pub sec: u16,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Thing {
    pub x: i16,
    pub y: i16,
    pub angle: i16,
    pub ednum: i16,
    pub flags: i16,
}

#[derive(Debug, Clone, Copy)]
pub enum Format {
    Vanilla,
    Hexen,
    PSX,
    Doom64,
}

pub struct Map {
    pub name: String,
    pub format: Format,
    pub vertices: Vec<Vertex>,
    pub lines: Vec<Linedef>,
    pub sides: Vec<Sidedef>,
    pub things: Vec<Thing>,
    pub sectors: Vec<Sector>,
}

pub fn load_map(lump: usize, wad: &DoomWad) -> Option<Map> {
    // The lump must have all of the required lumps following it, and it must
    // NOT BE one of the lumps that makes up a Doom map.
    let map_head_lump = &wad.lumps[lump];
    if lumps::ALL_LUMPS.iter().any(|&n| map_head_lump.name == n) {
        return None;
    }
    let map_lump_slice = {
        let start = lump;
        // Where is the first lump NOT in the ALL_LUMPS array?
        let end = wad.lumps.iter().skip(lump + 1).position(|wlump| {
            lumps::ALL_LUMPS.iter().any(|&n| wlump.name != n)
        }).unwrap_or(wad.lumps.len());
        &wad.lumps[start..end]
    };
    // Ensure all required lumps are present
    if !lumps::REQUIRED_LUMPS.iter().all(|&lumpn| map_lump_slice.iter().map(|lump| &lump.name).any(|name| name == lumpn)) {
        return None;
    }
    let vertexes_lump = map_lump_slice.iter().filter(|lump| lump.name == "VERTEXES").next()?;
    let linedefs_lump = map_lump_slice.iter().filter(|lump| lump.name == "LINEDEFS").next()?;
    let sidedefs_lump = map_lump_slice.iter().filter(|lump| lump.name == "SIDEDEFS").next()?;
    let things_lump = map_lump_slice.iter().filter(|lump| lump.name == "THINGS").next()?;
    let sectors_lump = map_lump_slice.iter().filter(|lump| lump.name == "SECTORS").next()?;
    None
}

fn load_vertexes(lump: &DoomWadLump, map_format: Format) -> Vec<Vertex> {
    let mut short_buf: [u8; 2] = [0; 2];
    let mut int_buf: [u8; 4] = [0; 4];
    let mut reader = Cursor::new(&lump.data);
    let vertex_count = lump.data.len() / match map_format {
        Format::Vanilla => 4, // Two 16-bit signed integers
        Format::Hexen => 4, // Two 16-bit signed integers
        Format::PSX => 8, // Two 32-bit signed integers
        Format::Doom64 => 8, // Two 32-bit signed integers
    };
    let vertices = (0..vertex_count).map(|_vertex_index| {
        reader.read_exact(match map_format {
            Format::Vanilla => &mut short_buf,
            Format::Hexen => &mut short_buf,
            Format::PSX => &mut int_buf,
            Format::Doom64 => &mut int_buf,
        }).ok()?;
        let x = i16::from_le_bytes(short_buf);
        reader.read_exact(match map_format {
            Format::Vanilla => &mut short_buf,
            Format::Hexen => &mut short_buf,
            Format::PSX => &mut int_buf,
            Format::Doom64 => &mut int_buf,
        }).ok()?;
        let y = i16::from_le_bytes(short_buf);
        Some(Vertex {x, y,})
    }).collect::<Option<Vec<Vertex>>>();
    vertices.unwrap()
}

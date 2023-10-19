//! Structures for the original Doom map format
use crate::wad::{DoomWad, DoomWadLump, LumpName};
use std::{
    error::Error,
    sync::Arc,
    mem,
    num::NonZeroUsize,
    io::{Cursor, Read},
};
use bitflags::bitflags;
use binrw::BinRead;
mod lumps;
#[cfg(feature="console")]
mod console;

#[derive(Debug, Clone, BinRead, PartialEq, Eq)]
#[br(little)]
pub struct Vertex {
    pub x: i16,
    pub y: i16
}

#[derive(Debug, Clone, BinRead, PartialEq, Eq)]
#[br(little)]
pub struct Linedef {
    pub v1: u16,
    pub v2: u16,
    pub flags: LinedefFlags,
    pub special: u16,
    pub tag: u16,
    pub front: u16,
    // #[br(parse_with(Linedef::backside))]
    pub back: u16,
}

// Making "back" an option adds an extra byte to the Linedef struct,
// which makes it have a different size than the vanilla doom Linedef struct
/* impl Linedef {
    #[binrw::parser(reader, endian)]
    fn backside() -> BinResult<Option<u16>> {
        let mut num_buf = [0; 2];
        let parse_fn = match endian {
            binrw::Endian::Big => i16::from_be_bytes,
            binrw::Endian::Little => i16::from_le_bytes
        };
        reader.read_exact(&mut num_buf)?;
        let value = match parse_fn(num_buf) {
            -1 => None,
            _ => {
                // The value was interpreted at an i16 so that -1 could be
                // used as a sentinel value, indicating nothing
                let parse_fn = match endian {
                    binrw::Endian::Big => u16::from_be_bytes,
                    binrw::Endian::Little => u16::from_le_bytes
                };
                Some(parse_fn(num_buf))
            }
        };
        Ok(value)
    }
} */

bitflags!{
    /// Linedef flags. See https://doomwiki.org/wiki/Linedef#Linedef_flags
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct LinedefFlags: u16 {
        const BLOCK_PLAYERS = 0x01;
        const BLOCK_MONSTERS = 0x02;
        const TWO_SIDED = 0x04;
        const UPPER_UNPEGGED = 0x08;
        const LOWER_UNPEGGED = 0x10;
        const AUTOMAP_SOLID = 0x20;
        const BLOCK_SOUND = 0x40;
        const AUTOMAP_HIDDEN = 0x80;
        const AUTOMAP_SHOWN = 0x100;
    }
}

impl BinRead for LinedefFlags {
    type Args<'a> = ();

    fn read_options<R: Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let mut num_buf: [u8; 2] = [0; 2];
        reader.read(&mut num_buf)?;
        let num = match endian {
            binrw::Endian::Big => u16::from_be_bytes(num_buf),
            binrw::Endian::Little => u16::from_le_bytes(num_buf),
        };
        Ok(LinedefFlags::from_bits_retain(num))
    }
}

#[derive(Debug, Clone, BinRead, PartialEq, Eq)]
#[br(little)]
pub struct Sidedef {
    pub x: i16,
    pub y: i16,
    pub upper: [u8; 8],
    pub lower: [u8; 8],
    pub middle: [u8; 8],
    pub sec: u16,
}

#[derive(Debug, Clone, BinRead, PartialEq, Eq)]
#[br(little)]
pub struct Sector {
    /// Floor height
    pub florh: i16,
    /// Ceiling height
    pub ceilh: i16,
    /// Floor material
    pub flort: [u8; 8],
    /// Ceiling material
    pub ceilt: [u8; 8],
    pub light: i16,
    pub special: i16,
    pub tag: i16,
}

#[derive(Debug, Clone, BinRead, PartialEq, Eq)]
#[br(little)]
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

/* 
pub enum BspData {
    Vanilla {
        nodes: Option<Arc<DoomWadLump>>
    },
    ZDoom {
        znodes: Option<Arc<DoomWadLump>>
    }
}
 */

#[derive(Debug, Clone)]
pub struct Map {
    pub name: LumpName,
    pub format: Format,
    // bsp: bool,
    things: Arc<DoomWadLump>,
    linedefs: Arc<DoomWadLump>,
    sidedefs: Arc<DoomWadLump>,
    vertexes: Arc<DoomWadLump>,
    sectors: Arc<DoomWadLump>,
}

impl Map {
    pub fn vertices(&self) -> Result<Vec<Vertex>, Box<dyn Error>> {
        let lump = &self.vertexes;
        let mut cursor = Cursor::new(&lump.data);
        lump.data.chunks_exact(mem::size_of::<Vertex>()).map(|_| {
            Vertex::read(&mut cursor).map_err(Box::from)
        }).collect()
    }

    pub fn linedefs(&self) -> Result<Vec<Linedef>, Box<dyn Error>> {
        let lump = &self.linedefs;
        let mut cursor = Cursor::new(&lump.data);
        lump.data.chunks_exact(mem::size_of::<Linedef>()).map(|_| {
            Linedef::read(&mut cursor).map_err(Box::from)
        }).collect()
    }

    pub fn sidedefs(&self) -> Result<Vec<Sidedef>, Box<dyn Error>> {
        let lump = &self.sidedefs;
        let mut cursor = Cursor::new(&lump.data);
        lump.data.chunks_exact(mem::size_of::<Sidedef>()).map(|_| {
            Sidedef::read(&mut cursor).map_err(Box::from)
        }).collect()
    }

    pub fn sectors(&self) -> Result<Vec<Sector>, Box<dyn Error>> {
        let lump = &self.sectors;
        let mut cursor = Cursor::new(&lump.data);
        lump.data.chunks_exact(mem::size_of::<Sector>()).map(|_| {
            Sector::read(&mut cursor).map_err(Box::from)
        }).collect()
    }

    pub fn things(&self) -> Result<Vec<Thing>, Box<dyn Error>> {
        let lump = &self.things;
        let mut cursor = Cursor::new(&lump.data);
        lump.data.chunks_exact(mem::size_of::<Thing>()).map(|_| {
            Thing::read(&mut cursor).map_err(Box::from)
        }).collect()
    }
}

/* 
#[derive(Debug, Clone, Error)]
pub enum FindMapError {
    #[error("Required lump {0} not found in map!")]
    MissingRequiredLump(LumpName),
}
 */

/// Look for all the maps in the WAD.
/// 
/// Returns a vector of information structs with information about the maps
pub fn find_maps(wad: &DoomWad, lump: Option<usize>) -> Vec<Map> {
    let start_index = lump.unwrap_or_default();
    let lumps = &wad.lumps[start_index..];
    // Manual implementation of slice::windows since that iterator stops before
    // the end
    let max_lump = lumps.len();
    (0..max_lump - lumps::MIN_LUMP_COUNT + 1).map(|index| {
        &lumps[index..(index + lumps::MAX_LUMP_COUNT).min(max_lump)]
    }).filter_map(|map_maybe| {
        // Heuristic/optimization: The first lump after the map name is THINGS
        if map_maybe.get(1)?.name != LumpName(*b"THINGS\0\0") { return None; }
        let map_lump_names: Vec<LumpName> = map_maybe.iter()
            .map(|lump| lump.name).collect();
        let name = map_lump_names[0];
        let map_lump_names = &map_lump_names[1..];
        // Cut map name...
        let map_maybe = &map_maybe[1..];
        // ...and lumps from other maps
        let map_maybe = {
            // Look for first lump which is NOT a map lump
            let outer_lump = map_lump_names.iter()
                .position(|name| !lumps::ALL_MAP_LUMPS.contains(&name))
                .unwrap_or(map_lump_names.len());
            let outer_lump = NonZeroUsize::new(outer_lump)?.get();
            &map_maybe[..outer_lump]
        };
        // Since map_maybe was modified...
        let map_lump_names: Vec<LumpName> = map_maybe.iter()
            .map(|lump| lump.name).collect();
        // Does it have all the required lumps for a Doom format map?
        let is_doom = map_lump_names.starts_with(&lumps::DOOM_START);
        let is_doom = is_doom && map_lump_names.contains(&lumps::DOOM_SECTORS);
        if !is_doom { return None; }
        let mut format = Format::Vanilla;
        // Check for complete vanilla map
        let bsp = map_lump_names.starts_with(&lumps::DOOM_VANILLA);
        // The SECTORS lump is in a weird position, but if the BSP lumps are
        // omitted, it comes right after the SIDEDEFS lump
        let sectors_index = if bsp {
            7  // See lumps.rs
        } else {
            map_lump_names.iter()
            .copied()
            .position(|name| name == lumps::DOOM_SECTORS)
            .unwrap()
        };
        // Check for other map formats
        if map_lump_names.ends_with(&[lumps::HEXEN_END]) || map_lump_names.ends_with(&lumps::HEXEN_END_OPTIONAL) {
            format = Format::Hexen;
        }
        if map_lump_names.ends_with(&lumps::PSX_END) {
            format = Format::PSX;
        }
        if map_lump_names.ends_with(&lumps::D64_END) {
            format = Format::Doom64;
        }
        // See lumps::DOOM_VANILLA
        let things = Arc::clone(&map_maybe[0]);
        let linedefs = Arc::clone(&map_maybe[1]);
        let sidedefs = Arc::clone(&map_maybe[2]);
        let vertexes = Arc::clone(&map_maybe[3]);
        let sectors = Arc::clone(&map_maybe[sectors_index]);
        Some(Map {
            name,
            format,
            things,
            linedefs,
            sidedefs,
            vertexes,
            sectors,
        })
    }).collect()
}

pub fn is_map(name: LumpName) -> bool {
    const MAP: [u8; 3] = [b'M', b'A', b'P'];
    if &name.0[0..3] == &MAP {
        // MAPxx (Doom II, Hexen, Strife, Doom 64)
        let mapnum = &name.as_str()[3..];
        mapnum.len() > 0 && mapnum.chars().all(|c| c.is_ascii_digit())
    } else if name.0[0] == b'E' && name.0[2] == b'M' {
        // ExMx (Doom, Heretic)
        let (episode, mapnum) = (name.0[1], name.0[3]);
        episode.is_ascii_digit() && mapnum.is_ascii_digit()
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod the_bronze_room;
    #[test]
    fn bronze_room() -> Result<(), Box<dyn Error>> {
        let the_wad = futures::executor::block_on(DoomWad::load("tests/data/the bronze room.wad"))?;
        let map_name = LumpName(*b"MAP01\0\0\0");
        let maps = find_maps(&the_wad, None);
        assert_eq!(maps.len(), 1);
        let map = maps.iter().filter(|m| m.name == map_name).next().ok_or(format!("Map {map_name} not found"))?;
        let linedefs = map.linedefs()?;
        assert_eq!(linedefs.as_slice(), the_bronze_room::LINEDEFS.as_slice());
        let sidedefs = map.sidedefs()?;
        assert_eq!(sidedefs.as_slice(), the_bronze_room::SIDEDEFS.as_slice());
        let vertices = map.vertices()?;
        assert_eq!(vertices.as_slice(), the_bronze_room::VERTICES.as_slice());
        let things = map.things()?;
        assert_eq!(things.as_slice(), the_bronze_room::THINGS.as_slice());
        let sectors = map.sectors()?;
        assert_eq!(sectors.as_slice(), the_bronze_room::SECTORS.as_slice());
        Ok(())
    }
}

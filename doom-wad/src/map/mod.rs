//! Structures for the original Doom map format
use crate::{
	wad::{DoomWad, DoomWadLump, LumpName},
	util::ReadFromReader,
};
use std::{
    error::Error,
    sync::Arc,
    mem,
    io::{Cursor, Read, Result as IOResult},
};
use bitflags::bitflags;
mod lumps;
#[cfg(feature="console")]
mod console;

#[derive(Debug, Clone)]
pub struct Vertex {
	pub x: i16,
	pub y: i16
}

impl ReadFromReader for Vertex {
    fn read(reader: &mut impl Read) -> IOResult<Self> {
        let mut num_buffer: [u8; 2] = [0; 2];
		reader.read_exact(&mut num_buffer)?;
		let x = i16::from_le_bytes(num_buffer);
		reader.read_exact(&mut num_buffer)?;
		let y = i16::from_le_bytes(num_buffer);
		Ok(Vertex { x, y })
    }
}

#[derive(Debug, Clone)]
pub struct Linedef {
	pub a: u16,
	pub b: u16,
	pub flags: LinedefFlags,
	pub special: u16,
	pub tag: u16,
	pub front: u16,
	pub back: u16,
}

impl ReadFromReader for Linedef {
    fn read(reader: &mut impl Read) -> IOResult<Self> {
        let mut num_buffer: [u8; 2] = [0; 2];
		reader.read_exact(&mut num_buffer)?;
		let a = u16::from_le_bytes(num_buffer);
		reader.read_exact(&mut num_buffer)?;
		let b = u16::from_le_bytes(num_buffer);
		reader.read_exact(&mut num_buffer)?;
		let flags = LinedefFlags::from_bits_retain(u16::from_le_bytes(num_buffer));
		reader.read_exact(&mut num_buffer)?;
		let special = u16::from_le_bytes(num_buffer);
		reader.read_exact(&mut num_buffer)?;
		let tag = u16::from_le_bytes(num_buffer);
		reader.read_exact(&mut num_buffer)?;
		let front = u16::from_le_bytes(num_buffer);
		reader.read_exact(&mut num_buffer)?;
		let back = u16::from_le_bytes(num_buffer);
		Ok(Linedef { a, b, flags, special, tag, front, back })
    }
}

bitflags!{
    /// Linedef flags. See https://doomwiki.org/wiki/Linedef#Linedef_flags
    #[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
pub struct Sidedef {
	pub x: i16,
	pub y: i16,
	pub upper: [u8; 8],
	pub lower: [u8; 8],
	pub middle: [u8; 8],
	pub sec: u16,
}

impl ReadFromReader for Sidedef {
    fn read(reader: &mut impl Read) -> IOResult<Self> {
        let mut num_buffer: [u8; 2] = [0; 2];
        let mut name_buffer: [u8; 8] = [0; 8];
		reader.read_exact(&mut num_buffer)?;
		let x = i16::from_le_bytes(num_buffer);
		reader.read_exact(&mut num_buffer)?;
		let y = i16::from_le_bytes(num_buffer);
		reader.read_exact(&mut name_buffer)?;
		let upper = name_buffer;
		reader.read_exact(&mut name_buffer)?;
		let lower = name_buffer;
		reader.read_exact(&mut name_buffer)?;
		let middle = name_buffer;
		reader.read_exact(&mut num_buffer)?;
		let sec = u16::from_le_bytes(num_buffer);
        Ok(Sidedef { x, y, upper, lower, middle, sec })
    }
}

#[derive(Debug, Clone)]
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

impl ReadFromReader for Sector {
    fn read(reader: &mut impl Read) -> IOResult<Self> {
        let mut num_buffer: [u8; 2] = [0; 2];
        let mut name_buffer: [u8; 8] = [0; 8];
		reader.read_exact(&mut num_buffer)?;
		let florh = i16::from_le_bytes(num_buffer);
		reader.read_exact(&mut num_buffer)?;
		let ceilh = i16::from_le_bytes(num_buffer);
		reader.read_exact(&mut name_buffer)?;
		let flort = name_buffer;
		reader.read_exact(&mut name_buffer)?;
		let ceilt = name_buffer;
		reader.read_exact(&mut num_buffer)?;
		let light = i16::from_le_bytes(num_buffer);
		reader.read_exact(&mut num_buffer)?;
		let special = i16::from_le_bytes(num_buffer);
		reader.read_exact(&mut num_buffer)?;
		let tag = i16::from_le_bytes(num_buffer);
        Ok(Sector { florh, ceilh, flort, ceilt, light, special, tag })
    }
}

#[derive(Debug, Clone)]
pub struct Thing {
	pub x: i16,
	pub y: i16,
	pub angle: i16,
	pub ednum: i16,
	pub flags: i16,
}

impl ReadFromReader for Thing {
    fn read(reader: &mut impl Read) -> IOResult<Self> {
        let mut num_buffer: [u8; 2] = [0; 2];
		reader.read_exact(&mut num_buffer)?;
		let x = i16::from_le_bytes(num_buffer);
		reader.read_exact(&mut num_buffer)?;
		let y = i16::from_le_bytes(num_buffer);
		reader.read_exact(&mut num_buffer)?;
		let angle = i16::from_le_bytes(num_buffer);
		reader.read_exact(&mut num_buffer)?;
		let ednum = i16::from_le_bytes(num_buffer);
		reader.read_exact(&mut num_buffer)?;
		let flags = i16::from_le_bytes(num_buffer);
        Ok(Thing { x, y, angle, ednum, flags })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Format {
	Vanilla,
	Hexen,
	PSX,
	Doom64,
}

#[derive(Clone)]
pub struct Map {
	pub name: LumpName,
	pub format: Format,
	pub lumps: Vec<Arc<DoomWadLump>>,
}

impl Map {
	pub fn vertices(&self) -> Result<Vec<Vertex>, Box<dyn Error>> {
		const LUMP_NAME: LumpName = LumpName(*b"VERTEXES");
		let lump = self.lumps.iter().find(|lump| lump.name == LUMP_NAME)
			.expect("All maps MUST have a VERTEXES lump!");
		let mut cursor = Cursor::new(&lump.data);
		lump.data.chunks_exact(mem::size_of::<Vertex>()).map(|_| {
			Vertex::read(&mut cursor).map_err(Box::from)
		}).collect()
	}

	pub fn linedefs(&self) -> Result<Vec<Linedef>, Box<dyn Error>> {
		const LUMP_NAME: LumpName = LumpName(*b"LINEDEFS");
		let lump = self.lumps.iter().find(|lump| lump.name == LUMP_NAME)
			.expect("All maps MUST have a LINEDEFS lump!");
		let mut cursor = Cursor::new(&lump.data);
		lump.data.chunks_exact(mem::size_of::<Linedef>()).map(|_| {
			Linedef::read(&mut cursor).map_err(Box::from)
		}).collect()
	}

	pub fn sidedefs(&self) -> Result<Vec<Sidedef>, Box<dyn Error>> {
		const LUMP_NAME: LumpName = LumpName(*b"SIDEDEFS");
		let lump = self.lumps.iter().find(|lump| lump.name == LUMP_NAME)
			.expect("All maps MUST have a SIDEDEFS lump!");
		let mut cursor = Cursor::new(&lump.data);
		lump.data.chunks_exact(mem::size_of::<Sidedef>()).map(|_| {
			Sidedef::read(&mut cursor).map_err(Box::from)
		}).collect()
	}

	pub fn sectors(&self) -> Result<Vec<Sector>, Box<dyn Error>> {
		const LUMP_NAME: LumpName = LumpName(*b"SECTORS\0");
		let lump = self.lumps.iter().find(|lump| lump.name == LUMP_NAME)
			.expect("All maps MUST have a SECTORS lump!");
		let mut cursor = Cursor::new(&lump.data);
		lump.data.chunks_exact(mem::size_of::<Sector>()).map(|_| {
			Sector::read(&mut cursor).map_err(Box::from)
		}).collect()
	}

	pub fn things(&self) -> Result<Vec<Thing>, Box<dyn Error>> {
		const LUMP_NAME: LumpName = LumpName(*b"THINGS\0\0");
		let lump = self.lumps.iter().find(|lump| lump.name == LUMP_NAME)
			.expect("All maps MUST have a THINGS lump!");
		let mut cursor = Cursor::new(&lump.data);
		lump.data.chunks_exact(mem::size_of::<Thing>()).map(|_| {
			Thing::read(&mut cursor).map_err(Box::from)
		}).collect()
	}
}

/// Check a lump to see whether it is a vanilla Doom map lump (or similar)
/// 
/// Returns the map name, the map format, and the slice of lumps which make up
/// the map. This does not read the map; that should be done by the application
/// which uses the output from this function
/// 
/// If the map does not have all required lumps, returns None
pub fn open_map(lump: usize, wad: &DoomWad) -> Option<Map> {
	// The lump must have all of the required lumps following it, and it must
	// NOT BE one of the lumps that makes up a Doom map.
	let map_head_lump = &wad.lumps[lump];
	if lumps::ALL_LUMPS.iter().any(|&n| map_head_lump.name == n) {
		return None;
	}
	let map_lump_slice: Vec<Arc<DoomWadLump>> = {
		let start = lump;
		// Where is the first lump NOT in the ALL_LUMPS array?
		let end = wad.lumps.iter().skip(lump + 1).position(|wlump| {
			lumps::ALL_LUMPS.iter().any(|&n| wlump.name != n)
		}).unwrap_or(wad.lumps.len());
		(&wad.lumps[start..end]).iter().map(|lu| Arc::clone(lu)).collect()
	};
	let map_lump_names: Box<[&LumpName]> = map_lump_slice.iter()
		.map(|lump| &lump.name).collect();
	// Make sure all required lumps are present
	if !map_lump_names.iter().all(|&ln| lumps::REQUIRED_LUMPS.iter()
		.any(|lln| ln == lln)) {
		return None;
	}
	// Find map format
	let format = if map_lump_names.iter().any(|&ln| ln == &lumps::HEXEN_LUMPS) {
		Format::Hexen
	} else if map_lump_names.iter().all(|&ln| lumps::D64_LUMPS.iter().any(|lln| ln == lln)) {
		Format::Doom64
	} else if map_lump_names.iter().all(|&ln| lumps::PSX_LUMPS.iter().any(|lln| ln == lln)) {
		Format::PSX
	} else {
		Format::Vanilla
	};
	Some(Map {
		name: map_head_lump.name.clone(),
		format,
		lumps: map_lump_slice
	})
}

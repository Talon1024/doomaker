//! Structures for the original Doom map format
use crate::wad::{DoomWad, DoomWadLump};
use std::{mem, io::{Read, Cursor}, error::Error};
mod lumps;
#[cfg(feature="console")]
mod console;

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
	pub upper: [u8; 8],
	pub lower: [u8; 8],
	pub middle: [u8; 8],
	pub sec: u16,
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

#[derive(Clone)]
pub struct Map<'a> {
	pub name: String,
	pub format: Format,
	pub lumps: &'a [DoomWadLump],
}

impl<'a> Map<'a> {
	pub fn vertices(&self) -> Result<Vec<Vertex>, Box<dyn Error>> {
		let lump = self.lumps.iter().find(|lump| lump.name == "VERTEXES")
			.expect("All maps MUST have a VERTEXES lump!");
		lump.data.chunks_exact(mem::size_of::<Vertex>()).map(|ch| {
			let mut cur = Cursor::new(ch);
			let mut numbuf: [u8; 2] = [0; 2];
			cur.read_exact(&mut numbuf)?;
			let x = i16::from_le_bytes(numbuf);
			cur.read_exact(&mut numbuf)?;
			let y = i16::from_le_bytes(numbuf);
			Ok(Vertex { x, y })
		}).collect()
	}

	pub fn linedefs(&self) -> Result<Vec<Linedef>, Box<dyn Error>> {
		let lump = self.lumps.iter().find(|lump| lump.name == "LINEDEFS")
			.expect("All maps MUST have a LINEDEFS lump!");
		lump.data.chunks_exact(mem::size_of::<Linedef>()).map(|ch| {
			let mut cur = Cursor::new(ch);
			let mut numbuf: [u8; 2] = [0; 2];
			cur.read_exact(&mut numbuf)?;
			let a = u16::from_le_bytes(numbuf);
			cur.read_exact(&mut numbuf)?;
			let b = u16::from_le_bytes(numbuf);
			cur.read_exact(&mut numbuf)?;
			let flags = u16::from_le_bytes(numbuf);
			cur.read_exact(&mut numbuf)?;
			let special = u16::from_le_bytes(numbuf);
			cur.read_exact(&mut numbuf)?;
			let tag = u16::from_le_bytes(numbuf);
			cur.read_exact(&mut numbuf)?;
			let front = u16::from_le_bytes(numbuf);
			cur.read_exact(&mut numbuf)?;
			let back = u16::from_le_bytes(numbuf);
			Ok(Linedef {
				a,
				b,
				flags,
				special,
				tag,
				front,
				back,
			})
		}).collect()
	}

	pub fn sidedefs(&self) -> Result<Vec<Sidedef>, Box<dyn Error>> {
		let lump = self.lumps.iter().find(|lump| lump.name == "SIDEDEFS")
			.expect("All maps MUST have a SIDEDEFS lump!");
		lump.data.chunks_exact(mem::size_of::<Sidedef>()).map(|ch| {
			let mut cur = Cursor::new(ch);
			let mut numbuf: [u8; 2] = [0; 2];
			let mut strbuf: [u8; 8] = [0; 8];
			cur.read_exact(&mut numbuf)?;
			let x = i16::from_le_bytes(numbuf);
			cur.read_exact(&mut numbuf)?;
			let y = i16::from_le_bytes(numbuf);
			cur.read_exact(&mut strbuf)?;
			let upper = strbuf;
			cur.read_exact(&mut strbuf)?;
			let middle = strbuf;
			cur.read_exact(&mut strbuf)?;
			let lower = strbuf;
			cur.read_exact(&mut numbuf)?;
			let sec = u16::from_le_bytes(numbuf);
			Ok(Sidedef {
				x,
				y,
				upper,
				middle,
				lower,
				sec
			})
		}).collect()
	}

	pub fn sectors(&self) -> Result<Vec<Sector>, Box<dyn Error>> {
		let lump = self.lumps.iter().find(|lump| lump.name == "SECTORS")
			.expect("All maps MUST have a SECTORS lump!");
		lump.data.chunks_exact(mem::size_of::<Sector>()).map(|ch| {
			let mut cur = Cursor::new(ch);
			let mut numbuf: [u8; 2] = [0; 2];
			let mut strbuf: [u8; 8] = [0; 8];
			cur.read_exact(&mut numbuf)?;
			let florh = i16::from_le_bytes(numbuf);
			cur.read_exact(&mut numbuf)?;
			let ceilh = i16::from_le_bytes(numbuf);
			cur.read_exact(&mut strbuf)?;
			let flort = strbuf;
			cur.read_exact(&mut strbuf)?;
			let ceilt = strbuf;
			cur.read_exact(&mut numbuf)?;
			let light = i16::from_le_bytes(numbuf);
			cur.read_exact(&mut numbuf)?;
			let special = i16::from_le_bytes(numbuf);
			cur.read_exact(&mut numbuf)?;
			let tag = i16::from_le_bytes(numbuf);
			Ok(Sector {
				florh,
				ceilh,
				flort,
				ceilt,
				light,
				special,
				tag
			})
		}).collect()
	}

	pub fn things(&self) -> Result<Vec<Thing>, Box<dyn Error>> {
		let lump = self.lumps.iter().find(|lump| lump.name == "THINGS")
			.expect("All maps MUST have a THINGS lump!");
		lump.data.chunks_exact(mem::size_of::<Thing>()).map(|ch| {
			let mut cur = Cursor::new(ch);
			let mut numbuf: [u8; 2] = [0; 2];
			cur.read_exact(&mut numbuf)?;
			let x = i16::from_le_bytes(numbuf);
			cur.read_exact(&mut numbuf)?;
			let y = i16::from_le_bytes(numbuf);
			cur.read_exact(&mut numbuf)?;
			let angle = i16::from_le_bytes(numbuf);
			cur.read_exact(&mut numbuf)?;
			let ednum = i16::from_le_bytes(numbuf);
			cur.read_exact(&mut numbuf)?;
			let flags = i16::from_le_bytes(numbuf);
			Ok(Thing {
				x,
				y,
				angle,
				ednum,
				flags,
			})
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
	let map_lump_slice = {
		let start = lump;
		// Where is the first lump NOT in the ALL_LUMPS array?
		let end = wad.lumps.iter().skip(lump + 1).position(|wlump| {
			lumps::ALL_LUMPS.iter().any(|&n| wlump.name != n)
		}).unwrap_or(wad.lumps.len());
		&wad.lumps[start..end]
	};
	let map_lump_names: Box<[&String]> = map_lump_slice.iter().map(|lump| &lump.name).collect();
	// Make sure all required lumps are present
	if !map_lump_names.iter().all(|&ln| lumps::REQUIRED_LUMPS.iter().any(|&lln| ln == lln)) {
		return None;
	}
	// Find map format
	let format = if map_lump_names.iter().any(|&ln| ln == lumps::HEXEN_LUMPS) {
		Format::Hexen
	} else if map_lump_names.iter().all(|&ln| lumps::D64_LUMPS.iter().any(|&lln| ln == lln)) {
		Format::Doom64
	} else if map_lump_names.iter().all(|&ln| lumps::PSX_LUMPS.iter().any(|&lln| ln == lln)) {
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

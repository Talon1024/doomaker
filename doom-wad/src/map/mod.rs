// Structures for the original Doom map format
use crate::wad::{DoomWad, DoomWadLump};
mod lumps;

/* 
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
 */

#[derive(Debug, Clone, Copy)]
pub enum Format {
	Vanilla,
	Hexen,
	PSX,
	Doom64,
}

pub struct Map<'a> {
	pub name: String,
	pub format: Format,
	pub lumps: &'a [DoomWadLump],
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

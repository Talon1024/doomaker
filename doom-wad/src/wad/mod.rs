pub mod lump_name;

use std::{
	io::*,
	fs::{File, read},
	ops::{Deref, DerefMut},
	result::Result,
	str::from_utf8,
	error::Error,
	fmt::{Display, Formatter}, collections::HashMap
};
use ahash::RandomState;
use lump_name::LumpName;

const IWAD_HEADER: &str = "IWAD";
const PWAD_HEADER: &str = "PWAD";

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DoomWadType {
	IWAD,
	PWAD,
	Invalid,
}
#[derive(Debug, Clone)]
pub struct DoomWadLump {
	pub name: LumpName,
	pub data: Vec<u8>,
}
#[derive(Debug, Clone)]
pub struct DoomWad {
	pub wtype: DoomWadType,
	pub lumps: Vec<DoomWadLump>,
}
#[derive(Debug, Clone)]
struct DoomWadDirEntry {
	name: LumpName,
	pos: u64,
	size: usize,
}

#[derive(Debug, Clone)]
struct InvalidWadError(String);
impl Display for InvalidWadError {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		writeln!(f, "Invalid WAD! Header is {}", self.0)?;
		Ok(())
	}
}
impl Error for InvalidWadError{}

impl DoomWad {
	pub fn load_sync(filename: &str) -> Result<DoomWad, Box<dyn Error>> {
		let file = read(filename)?;
		DoomWad::read_from(&file)
	}

	pub fn load(filename: &str) -> Result<DoomWad, Box<dyn Error>> {
		// TODO: Make asynchronous
		let file = read(filename)?;
		DoomWad::read_from(&file)
	}

	pub fn read_from(file: &[u8]) -> Result<DoomWad, Box<dyn Error>> {
		let mut wad: DoomWad = DoomWad {
			wtype: DoomWadType::Invalid,
			lumps: Vec::new()
		};
		let mut reader = BufReader::new(Cursor::new(file));
		let mut num_buffer: [u8; 4] = [0; 4];
		// Get WAD type
		reader.read_exact(&mut num_buffer)?;
		let header = from_utf8(&num_buffer)?;
		wad.wtype = match header {
			IWAD_HEADER => DoomWadType::IWAD,
			PWAD_HEADER => DoomWadType::PWAD,
			_ => DoomWadType::Invalid
		};
		if let DoomWadType::Invalid = wad.wtype {
			return Err(Box::new(InvalidWadError(header.to_owned())));
		}

		reader.read_exact(&mut num_buffer)?;
		let lump_count: usize = u32::from_le_bytes(num_buffer) as usize;
		wad.lumps.reserve(lump_count);
		reader.read_exact(&mut num_buffer)?;
		let directory_offset = u32::from_le_bytes(num_buffer);
		reader.seek(SeekFrom::Start(directory_offset as u64))?;

		// Read directory
		let mut directory: Vec<DoomWadDirEntry> = Vec::with_capacity(lump_count);
		for _lump_index in 0..lump_count {
			directory.push(DoomWad::read_directory_entry(&mut reader)?);
		}

		// Read each lump
		for dir_entry in directory.into_iter() {
			reader.seek(SeekFrom::Start(dir_entry.pos))?;
			let mut data: Vec<u8> = Vec::with_capacity(dir_entry.size);
			reader.read_exact(data.as_mut())?;
			wad.lumps.push(DoomWadLump{name: dir_entry.name, data: data });
		}
		Ok(wad)
	}

	fn read_directory_entry(reader: &mut BufReader<Cursor<&[u8]>>) -> Result<DoomWadDirEntry, Box<dyn Error>> {
		let mut num_buffer: [u8; 4] = [0; 4];
		let mut name_buffer: [u8; 8] = [0; 8];
		reader.read_exact(&mut num_buffer)?;
		let pos = u32::from_le_bytes(num_buffer) as u64;
		reader.read_exact(&mut num_buffer)?;
		let size = u32::from_le_bytes(num_buffer) as usize;
		reader.read_exact(&mut name_buffer)?;
		let name = LumpName::try_from(name_buffer.as_slice())?;
		return Ok(DoomWadDirEntry { name, pos, size });
	}

	pub fn write(&self, filename: &str) -> Result<(), Box<dyn Error>> {
		// TODO: Make asynchronous
		let mut data: Vec<u8> = Vec::<u8>::new();
		self.write_to(&mut data)?;
		let mut file = File::create(filename)?;
		file.write_all(&data[..])?;
		Ok(())
	}

	pub fn write_sync(&self, filename: &str) -> Result<(), Box<dyn Error>> {
		let mut data: Vec<u8> = Vec::<u8>::new();
		self.write_to(&mut data)?;
		let mut file = File::create(filename)?;
		file.write_all(&data[..])?;
		Ok(())
	}

	pub fn write_to(&self, file: &mut dyn Write) -> Result<(), Box<dyn Error>> {
		let header_size: u32 = 12;
		let mut num_buffer: [u8; 4] = [0; 4];
		let mut writer = BufWriter::new(file);
		let header = match self.wtype {
			DoomWadType::IWAD => IWAD_HEADER,
			DoomWadType::PWAD => PWAD_HEADER,
			DoomWadType::Invalid => {unreachable!("Attempted to write an invalid WAD!");}
		};
		num_buffer.copy_from_slice(header.as_bytes());
		writer.write(&num_buffer)?; // IWAD/PWAD header
		// Lump count
		num_buffer = (self.lumps.len() as u32).to_le_bytes();
		writer.write(&num_buffer)?;
		let directory_offset: u32 = {
			let all_lumps_size: u32 = self.lumps.iter()
				.map(|lump| lump.data.len() as u32).sum();
			header_size + all_lumps_size
		};
		// Directory offset
		num_buffer = directory_offset.to_le_bytes();
		writer.write(&num_buffer)?;
		// Directory info
		let mut directory: Vec<DoomWadDirEntry> = Vec::with_capacity(self.lumps.len());
		let mut current_pos: u64 = header_size as u64;
		for lump in self.lumps.iter() {
			// Lump data
			directory.push(DoomWadDirEntry{
				name: lump.name.clone(),
				pos: current_pos,
				size: lump.data.len()
			});
			writer.write(&lump.data)?;
			current_pos += lump.data.len() as u64;
		}
		for dir_entry in directory.iter() {
			let lump_name = dir_entry.name;
			num_buffer = (dir_entry.pos as u32).to_le_bytes();
			writer.write(&num_buffer)?;
			num_buffer = (dir_entry.size as u32).to_le_bytes();
			writer.write(&num_buffer)?;
			writer.write(lump_name.into())?;
		}
		Ok(())
	}
}

pub trait Namespaced<'a> {
	fn namespace(&'a self, namespace: &str) -> Vec<&'a DoomWadLump>;
}

#[derive(Debug, Default)]
pub struct DoomWadCollection(Vec<DoomWad>);

impl Deref for DoomWadCollection {
	type Target = Vec<DoomWad>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl DerefMut for DoomWadCollection {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl DoomWadCollection {
	fn lump_map(&self) -> HashMap<LumpName, &DoomWadLump, RandomState> {
		let mut lump_map = HashMap::<LumpName, &DoomWadLump, RandomState>::default();
		self.iter().for_each(|wad| {
			wad.lumps.iter().for_each(|lump| {
				lump_map.insert(lump.name.clone(), lump);
			})
		});
		lump_map
	}
}
// WIP!
/* 
impl<'a> Namespaced<'a> for DoomWadCollection {
	fn namespace(&'a self, namespace: &str) -> Vec<&'a DoomWadLump> {
		let bounds: (&[&str], &[&str]) = match namespace {
			"patches" => (&["P_START", "PP_START"], &["P_END", "PP_END"]),
			"flats" => (&["F_START", "FF_START"], &["F_END", "FF_END"]),
			"sprites" => (&["S_START"], &["S_END"])
		};
		let subsections: Option<(&[&str], &[&str])> = match namespace {
			"patches" => Some((&["P1_START", "P2_START", "P3_START"], &["P1_END", "P2_END", "P3_END"])),
			"flats" => Some((&["F1_START", "F2_START", "F3_START"], &["F1_END", "F2_END", "F3_END"])),
			"sprites" => None
		};
		self.iter().map(|wad| {
			let namespace_slices = wad.namespace_lumps(bounds, subsections);
		}).collect()
	}
}
 */
impl<'a> DoomWad {
	pub fn namespace_lumps(&'a self, ns: (&[&str], &[&str]), sub: Option<(&[&str], &[&str])>) -> Vec<&'a DoomWadLump> {
		let ns_index = self.lumps.iter().position(
			|lu| ns.0.iter().any(|n| n == &lu.name));
		if ns_index.is_none() {
			return vec![];
		}
		let ns_index = ns_index.unwrap() + 1;
		let ns_endindex = self.lumps.iter().skip(ns_index).position(
			|lu| ns.1.iter().any(|n| n == &lu.name));
		if ns_endindex.is_none() {
			return vec![];
		}
		let ns_endindex = ns_index + ns_endindex.unwrap() - 1;
		let ns_slice = &self.lumps[ns_index..ns_endindex];
		let has_subsections = sub.is_some() && {
			// The first lump after a namespace start marker can be a
			// subsection start marker
			match ns_slice.iter().next() {
				Some(lu) => {
					sub.unwrap().0.iter().any(|&n| n == &lu.name)
				},
				None => false,
			}
		};
		if has_subsections {
			// Should be a vector of all the subsection slices
			let sub = sub.unwrap();
			let sub = sub.0.iter().chain(sub.1);
			ns_slice.iter().filter_map(|lu| {
				if sub.clone().any(|ln| ln == &lu.name) {
					None
				} else {
					Some(lu)
				}
			}).collect()
		} else {
			ns_slice.iter().map(|lu| lu).collect()
		}
	}
	pub fn ns_patches(&'a self) -> Vec<&'a DoomWadLump> {
		let patches_start = [b"P_START", b"PP_START"].map(LumpName::try_from);
		let subsect_start = [b"P1_START", b"P2_START", b"P3_START"];
		let patches_end = [b"P_END", b"PP_END"];
		let subsect_end = [b"P1_END", b"P2_END", b"P3_END"];
		self.namespace_lumps((&patches_start, &patches_end),
			Some((&subsect_start, &subsect_end)))
	}/* 
	pub fn ns_flats(&'a self) -> Vec<&'a [DoomWadLump]> {
		let flats_start = ["F_START", "FF_START"];
		let subsect_start = ["F1_START", "F2_START", "F3_START"];
		let flats_end = ["F_END", "FF_END"];
		let subsect_end = ["F1_END", "F2_END", "F3_END"];
		let a = self.lumps.iter().enumerate().filter(
			|&(_i, lu)| lu.name == "FF_START" || lu.name == "F_START");
		let b = self.lumps.iter().skip(a).position(|lu| lu.name == "P_END" || lu.name == "PP_END")? - 1;
		vec![]
	} */
}

#[cfg(test)]
mod tests {
	use super::*;

	macro_rules! empty_lump {
		($name:expr) => {
			DoomWadLump {
				name: LumpName::try_from($name).unwrap(),
				data: vec![]
			}
		};
	}

	#[test]
	fn basic_namespace() -> Result<(), Box<dyn Error>> {
		let wad = DoomWad {
			wtype: DoomWadType::PWAD,
			lumps: vec![
				empty_lump!("P_START"),
				empty_lump!("PSTONE2"),
				empty_lump!("PIVY3"),
				empty_lump!("GNYX"),
				empty_lump!("EBG13"),
				empty_lump!("NUTS"),
				empty_lump!("CRAPPED"),
				empty_lump!("P_END"),
			],
		};
		let expected: Vec<&DoomWadLump> = (1..7).map(|index| {
			&wad.lumps[index]
		}).collect();
		let actual = wad.ns_patches();
		expected.into_iter().zip(actual).for_each(|(exp, act)| {
			assert_eq!(exp.name, act.name);
		});
		Ok(())
	}

	#[test]
	fn adv_namespace() -> Result<(), Box<dyn Error>> {
		let wad = DoomWad {
			wtype: DoomWadType::PWAD,
			lumps: vec![
				empty_lump!("P_START"),
				empty_lump!("P1_START"),
				empty_lump!("PSTONE2"),
				empty_lump!("PIVY3"),
				empty_lump!("P1_END"),
				empty_lump!("P2_START"),
				empty_lump!("GNYX"),
				empty_lump!("EBG13"),
				empty_lump!("P2_END"),
				empty_lump!("P3_START"),
				empty_lump!("NUTS"),
				empty_lump!("CRAPPED"),
				empty_lump!("P3_END"),
				empty_lump!("P_END"),
			],
		};
		let expected_slice: Vec<&DoomWadLump> =
		(2..4).chain(6..8).chain(10..12).map(
			|index| &wad.lumps[index]).collect();
		let actual_slice = wad.ns_patches();
		expected_slice.into_iter().zip(actual_slice).for_each(|(exp, act)| {
			assert_eq!(exp.name, act.name);
		});
		Ok(())
	}
}

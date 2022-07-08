mod lump_name;

use std::{
	io::*,
	fs::{File, read},
	result::Result,
	str::from_utf8,
	error::Error,
	fmt::{Display, Formatter}, collections::HashMap, path::Path
};
use ahash::RandomState;
pub use lump_name::{LumpName, LumpNameConvertError};
use derive_deref::*;

use crate::res::{TextureDefinitionsLumps, read_texturex};

const IWAD_HEADER: &str = "IWAD";
const PWAD_HEADER: &str = "PWAD";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoomWadType {
	IWAD,
	PWAD,
	Invalid,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoomWadLump {
	pub name: LumpName,
	pub data: Vec<u8>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoomWad {
	pub wtype: DoomWadType,
	pub lumps: Vec<DoomWadLump>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
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
	pub fn load_sync(filename: &(impl AsRef<Path> + ?Sized)) -> Result<DoomWad, Box<dyn Error>> {
		let file = read(filename)?;
		DoomWad::read_from(&file)
	}

	pub fn load(filename: &(impl AsRef<Path> + ?Sized)) -> Result<DoomWad, Box<dyn Error>> {
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
			let mut data: Vec<u8> = vec![0; dir_entry.size];
			reader.read_exact(data.as_mut())?;
			wad.lumps.push(DoomWadLump{name: dir_entry.name, data: data });
		}
		Ok(wad)
	}

	fn read_directory_entry(reader: &mut impl Read) -> Result<DoomWadDirEntry, Box<dyn Error>> {
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

	pub fn write(&self, filename: &(impl AsRef<Path> + ?Sized)) -> Result<(), Box<dyn Error>> {
		// TODO: Make asynchronous
		let mut data: Vec<u8> = Vec::<u8>::new();
		self.write_to(&mut data)?;
		let mut file = File::create(filename)?;
		file.write_all(&data[..])?;
		Ok(())
	}

	pub fn write_sync(&self, filename: &(impl AsRef<Path> + ?Sized)) -> Result<(), Box<dyn Error>> {
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
				name: lump.name,
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
			writer.write(&lump_name.0)?;
		}
		Ok(())
	}
}

pub trait Namespaced<'a> {
	fn namespace(&'a self, namespace: &str) -> Namespace<'a>;
}

#[derive(Debug, Default, Deref, DerefMut)]
pub struct DoomWadCollection(pub Vec<DoomWad>);

pub trait GetLump<'a> {
	fn get_lump(&'a self, lump_name: LumpName) -> Option<&'a DoomWadLump>;
}

pub type LumpMap<'a> = HashMap<LumpName, &'a DoomWadLump, RandomState>;

impl<'a> GetLump<'a> for DoomWadCollection {
	fn get_lump(&'a self, lump_name: LumpName) -> Option<&'a DoomWadLump> {
		self.0.iter().filter_map(|wad| {
			wad.lumps.iter().rfind(|lu| {
				lu.name == lump_name
			})
		}).next()
	}
}
impl<'a> GetLump<'a> for DoomWad {
	fn get_lump(&'a self, lump_name: LumpName) -> Option<&'a DoomWadLump> {
		self.lumps.iter().rfind(|lu| {
			lu.name == lump_name
		})
	}
}
impl<'a> GetLump<'a> for LumpMap<'a> {
	fn get_lump(&'a self, lump_name: LumpName) -> Option<&'a DoomWadLump> {
		self.get(&lump_name).copied()
	}
}

impl<'a> DoomWadCollection {
	pub fn lump_map(&'a self) -> LumpMap<'a> {
		let mut lump_map = LumpMap::default();
		self.0.iter().for_each(|wad| {
			wad.lumps.iter().for_each(|lump| {
				lump_map.insert(lump.name, lump);
			});
		});
		lump_map
	}
	pub fn get(
		&'a self,
		lump_name: LumpName,
		map: Option<&'a (dyn GetLump<'a>)>
	) -> Option<&'a DoomWadLump> {
		if let Some(map) = map {
			map.get_lump(lump_name)
		} else {
			self.get_lump(lump_name)
		}
	}
	pub fn playpal(
		&'a self, map: Option<&'a (dyn GetLump<'a>)>
	) -> Option<&'a DoomWadLump> {
		let playpal = LumpName(*b"PLAYPAL\0");
		self.get(playpal, map)
	}
	pub fn textures(
		&'a self, map: Option<&'a (dyn GetLump<'a>)>
	) -> Option<TextureDefinitionsLumps<'a>> {
		let pnames = LumpName(*b"PNAMES\0\0");
		let tex_lumps = [LumpName(*b"TEXTURE1"), LumpName(*b"TEXTURE2"),
			LumpName(*b"TEXTURE3")];
		let pnames = self.get(pnames, map)?;
		Some(TextureDefinitionsLumps(tex_lumps.iter().filter_map(|&name| {
			let lump = self.get(name, map)?;
			read_texturex(lump, pnames, map.unwrap_or(self)).ok()
		}).collect()))
	}
}

impl<'a> Namespaced<'a> for DoomWadCollection {
	fn namespace(&'a self, namespace: &str) -> Namespace<'a> {
		let namespaces = self.0.iter().map(|wad| wad.namespace(namespace).0);
		Namespace(namespaces.flatten().collect())
	}
}

impl<'a> Namespaced<'a> for DoomWad {
	fn namespace(&'a self, namespace: &str) -> Namespace<'a> {
		let bounds = match namespace {
			"patches" => Some((
				vec![LumpName(*b"P_START\0"), LumpName(*b"PP_START")],
				vec![LumpName(*b"P_END\0\0\0"), LumpName(*b"PP_END\0\0")])),
			"flats" => Some((
				vec![LumpName(*b"F_START\0"), LumpName(*b"FF_START")],
				vec![LumpName(*b"F_END\0\0\0"), LumpName(*b"FF_END\0\0")])),
			"sprites" => Some((
				vec![LumpName(*b"S_START\0")],
				vec![LumpName(*b"S_END\0\0\0")])),
			_ => None,
		};
		let bounds = bounds.as_ref().map(|(start, end)| {
			(start.as_slice(), end.as_slice())
		});
		let subsections = match namespace {
			"patches" => Some((
				vec![LumpName(*b"P1_START"), LumpName(*b"P2_START"),
					LumpName(*b"P3_START")],
				vec![LumpName(*b"P1_END\0\0"), LumpName(*b"P2_END\0\0"),
					LumpName(*b"P3_END\0\0")])),
			"flats" => Some((
				vec![LumpName(*b"F1_START"), LumpName(*b"F2_START"),
					LumpName(*b"F3_START")],
				vec![LumpName(*b"F1_END\0\0"), LumpName(*b"F2_END\0\0"),
					LumpName(*b"F3_END\0\0")])),
			_ => None
		};
		let subsections = subsections.as_ref().map(|(start, end)|{
			(start.as_slice(), end.as_slice())
		});
		if let Some(bounds) = bounds {
			self.namespace_lumps(bounds, subsections)
		} else {
			Namespace(vec![])
		}
	}
}

#[derive(Debug, Clone, Deref, DerefMut, PartialEq, Eq)]
pub struct Namespace<'a>(pub(crate) Vec<&'a DoomWadLump>);
impl<'a> Namespace<'a> {
	pub fn lump_map(&'a self) -> LumpMap<'a> {
		let mut lump_map = LumpMap::default();
		self.0.iter().for_each(|lump| {
			lump_map.insert(lump.name, lump);
		});
		lump_map
	}
	pub fn get(
		&'a self,
		lump_name: LumpName,
		map: Option<&'a (dyn GetLump<'a>)>
	) -> Option<&'a DoomWadLump> {
		if let Some(map) = map {
			map.get_lump(lump_name)
		} else {
			self.get_lump(lump_name)
		}
	}
}
impl<'a> GetLump<'a> for Namespace<'a> {
	fn get_lump(&'a self, lump_name: LumpName) -> Option<&'a DoomWadLump> {
		self.0.iter().find(|lu| {
			lu.name == lump_name
		}).copied()
	}
}

pub type NamespaceBounds<'a> = (&'a [LumpName], &'a [LumpName]);
impl<'a> DoomWad {
	pub fn namespace_lumps(&'a self,
		ns: NamespaceBounds,
		sub: Option<NamespaceBounds>
	) -> Namespace<'a> {
		let ns_index = self.lumps.iter().position(
			|lu| ns.0.iter().any(|&n| n == lu.name));
		if ns_index.is_none() {
			return Namespace(vec![]);
		}
		let ns_index = ns_index.unwrap() + 1;
		let ns_endindex = self.lumps.iter().skip(ns_index).position(
			|lu| ns.1.iter().any(|&n| n == lu.name));
		if ns_endindex.is_none() {
			return Namespace(vec![]);
		}
		let ns_endindex = ns_index + ns_endindex.unwrap();
		let ns_slice = &self.lumps[ns_index..ns_endindex];
		let has_subsections = sub.is_some() && {
			// The first lump after a namespace start marker can be a
			// subsection start marker
			match ns_slice.iter().next() {
				Some(lu) => {
					sub.unwrap().0.iter().any(|&n| n == lu.name)
				},
				None => false,
			}
		};
		Namespace(if has_subsections {
			// Should be a vector of all the subsection slices
			let sub = sub.unwrap();
			let sub = sub.0.iter().chain(sub.1);
			ns_slice.iter().filter_map(|lu| {
				if sub.clone().any(|&ln| ln == lu.name) {
					None
				} else {
					Some(lu)
				}
			}).collect()
		} else {
			ns_slice.iter().map(|lu| lu).collect()
		})
	}
	pub fn ns_patches(&'a self) -> Namespace<'a> {
		let patches_start = [b"P_START\0", b"PP_START"]
			.map(|fu| LumpName::try_from(fu.as_slice()).unwrap());
		let subsect_start = [b"P1_START", b"P2_START", b"P3_START"]
			.map(|fu| LumpName::try_from(fu.as_slice()).unwrap());
		let patches_end = [b"P_END\0", b"PP_END"]
			.map(|fu| LumpName::try_from(fu.as_slice()).unwrap());
		let subsect_end = [b"P1_END", b"P2_END", b"P3_END"]
			.map(|fu| LumpName::try_from(fu.as_slice()).unwrap());
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
		let expected = Namespace((1..7).map(|index| {
			&wad.lumps[index]
		}).collect());
		let actual = wad.ns_patches();
		assert_eq!(expected.len(), actual.len());
		assert_eq!(expected, actual);
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
		let expected_slice = Namespace(
		(2..4).chain(6..8).chain(10..12).map(
			|index| &wad.lumps[index]).collect());
		let actual_slice = wad.ns_patches();
		assert_eq!(expected_slice, actual_slice);
		Ok(())
	}
}

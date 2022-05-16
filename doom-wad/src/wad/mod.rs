pub mod util;

use std::io::*;
use std::fs::{File, read};
use std::result::Result;
use std::str::from_utf8;
use util::lump_name;
use util::to_lump_name;
use std::error::Error;
use std::fmt::{Display, Formatter};

const IWAD_HEADER: &str = "IWAD";
const PWAD_HEADER: &str = "PWAD";

pub enum DoomWadType {
	IWAD,
	PWAD,
	Invalid,
}
pub struct DoomWadLump {
	pub name: String,
	pub data: Vec<u8>,
}
pub struct DoomWad {
	pub wtype: DoomWadType,
	pub lumps: Vec<DoomWadLump>,
}
struct DoomWadDirEntry {
	name: String,
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
		let name = String::from_utf8(lump_name(&name_buffer))?;
		return Ok(DoomWadDirEntry { name: name, pos: pos, size: size });
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
			let lump_name = to_lump_name(&dir_entry.name);
			num_buffer = (dir_entry.pos as u32).to_le_bytes();
			writer.write(&num_buffer)?;
			num_buffer = (dir_entry.size as u32).to_le_bytes();
			writer.write(&num_buffer)?;
			writer.write(&lump_name)?;
		}
		Ok(())
	}
}

impl<'a> DoomWad {
	pub fn namespace(&'a self, ns: (&[&str], &[&str]), sub: Option<(&[&str], &[&str])>) -> Vec<&'a [DoomWadLump]> {
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
			let (sub_start, sub_end) = sub.unwrap();
			struct SubsectionIteration<'a> {
				start_index: usize,
				end_name: &'a str,
			}
			let sub_start_info: Vec<SubsectionIteration> = ns_slice.iter()
				.enumerate().filter_map(|(i, lu)| {
					let name_index = sub_start.iter().position(|n| n == &lu.name)?;
					let end_name = sub_end[name_index];
					Some(SubsectionIteration{
						start_index: i,
						end_name
					})
				}).collect();
			sub_start_info.iter().filter_map(|iteration| {
				// Position is relative from "skip" index
				let end_index = ns_slice.iter().skip(iteration.start_index)
				.position(|lu| { &lu.name == iteration.end_name})? +
				iteration.start_index;
				Some(&ns_slice[iteration.start_index..end_index])
			}).collect()
		} else {
			vec![ns_slice]
		}
	}
	pub fn ns_patches(&'a self) -> Vec<&'a [DoomWadLump]> {
		let patches_start = ["P_START", "PP_START"];
		let subsect_start = ["P1_START", "P2_START", "P3_START"];
		let patches_end = ["P_END", "PP_END"];
		let subsect_end = ["P1_END", "P2_END", "P3_END"];
		self.namespace((&patches_start, &patches_end),
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

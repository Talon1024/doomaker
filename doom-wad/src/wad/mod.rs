pub mod util;

use std::io::*;
use std::fs::File;
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
	pub fn load(filename: &str) -> Result<DoomWad, Box<dyn Error>> {
		let file = File::open(filename)?;
		DoomWad::read_from(&file)
	}

	pub fn read_from(file: &File) -> Result<DoomWad, Box<dyn Error>> {
		let mut wad: DoomWad = DoomWad {
			wtype: DoomWadType::Invalid,
			lumps: Vec::new()
		};
		let mut reader = BufReader::new(file);
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

	fn read_directory_entry(reader: &mut BufReader<&File>) -> Result<DoomWadDirEntry, Box<dyn Error>> {
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
		let file = File::create(filename)?;
		self.write_to(&file)
	}

	pub fn write_to(&self, file: &File) -> Result<(), Box<dyn Error>> {
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

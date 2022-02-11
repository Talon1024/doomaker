// TEXTURE1, TEXTURE2, and PNAMES
use crate::wad;
use crate::wad::util::*;
use std::error::Error;
use std::io::{Cursor, Read, Seek, SeekFrom};

pub struct TexturePatch {
	patch: String,
	x: i16, // X and Y offsets
	y: i16,
	flags: i32,
}
pub struct Texture {
	name: String,
	flags: i32,
	width: u16,
	height: u16,
	patches: Vec<TexturePatch>,
}
pub struct TextureDefinitions<'a> {
	textures: Vec<Texture>,
	lump: &'a wad::DoomWadLump,
	// https://users.rust-lang.org/t/hashmap-of-a-vector-of-objects/29220
	// by_name: HashMap<String, &'a Texture>
}

fn read_pnames(pnames: &wad::DoomWadLump) ->
	Result<Vec<String>, Box<dyn Error>>
{
	let mut num_buffer: [u8; 4] = [0; 4];
	let mut name_buffer: [u8; 8] = [0; 8];
	let mut pos = Cursor::new(&pnames.data);
	let name_count: usize = {
		pos.read_exact(&mut num_buffer)?;
		i32::from_le_bytes(num_buffer) as usize
	};
	let mut names: Vec<String> = Vec::with_capacity(name_count);
	(0..name_count).map(|_| -> Result<(), Box<dyn Error>> {
		pos.read_exact(&mut name_buffer)?;
		names.push(String::from_utf8(lump_name(&name_buffer))?);
		Ok(())
	}).collect::<Result<(), Box<dyn Error>>>()?;
	Ok(names)
}

pub fn read_texturex<'a>(wad: &'a wad::DoomWad, list: &'a wad::DoomWadLump, pnames: &wad::DoomWadLump) ->
	Result<TextureDefinitions<'a>, Box<dyn Error>>
{
	let patches = read_pnames(pnames)?;
	let mut pos = Cursor::new(&list.data);
	let mut name_buffer: [u8; 8] = [0; 8];
	let mut num_buffer: [u8; 4] = [0; 4];
	let mut short_buffer: [u8; 2] = [0; 2];
	let count: usize = {
		pos.read_exact(&mut num_buffer)?;
		i32::from_le_bytes(num_buffer) as usize
	};
	let mut tex_def_pos: Vec<u64> = vec![0; count];
	(0..count).map(|index| -> Result<(), Box<dyn Error>> {
		pos.read_exact(&mut num_buffer)?;
		tex_def_pos[index] = u32::from_le_bytes(num_buffer) as u64;
		Ok(())
	}).collect::<Result<(), Box<dyn Error>>>()?;
	let mut defs = TextureDefinitions {
		textures: Vec::with_capacity(count),
		lump: list,
		// by_name: HashMap::new(),
	};
	tex_def_pos.into_iter().try_for_each(|offset| -> Result<(), Box<dyn Error>> {
		pos.seek(SeekFrom::Start(offset))?;
		// Name (8 bytes)
		pos.read_exact(&mut name_buffer)?;
		let name = String::from_utf8(lump_name(&name_buffer))?;
		// Flags (4 bytes)
		pos.read_exact(&mut num_buffer)?;
		let flags = i32::from_le_bytes(num_buffer);
		pos.read_exact(&mut short_buffer)?;
		let width = u16::from_le_bytes(short_buffer);
		pos.read_exact(&mut short_buffer)?;
		let height = u16::from_le_bytes(short_buffer);
		pos.seek(SeekFrom::Current(4))?; // skip columndirectory
		pos.read_exact(&mut short_buffer)?;
		let patch_count = u16::from_le_bytes(short_buffer);
		let mut texture = Texture {
			name: name.clone(),
			flags: flags,
			width: width,
			height: height,
			patches: Vec::new(),
		};
		(0..patch_count ).map(|_| -> Result<(), Box<dyn Error>> {
			pos.read_exact(&mut short_buffer)?;
			let x = i16::from_le_bytes(short_buffer);
			pos.read_exact(&mut short_buffer)?;
			let y = i16::from_le_bytes(short_buffer);
			pos.read_exact(&mut short_buffer)?;
			let patch_name = patches[u16::from_le_bytes(short_buffer) as usize].clone();
			pos.read_exact(&mut num_buffer)?;
			let flags = i32::from_le_bytes(num_buffer);
			let patch = TexturePatch {
				patch: patch_name,
				x: x,
				y: y, 
				flags: flags
			};
			texture.patches.push(patch);
			Ok(())
		}).collect::<Result<(), Box<dyn Error>>>()?;
		defs.textures.push(texture);
		Ok(())
	})?;
	Ok(defs)
}

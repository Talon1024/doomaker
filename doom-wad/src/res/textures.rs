// TEXTURE1, TEXTURE2, and PNAMES
use crate::wad;
use crate::util;
use crate::wad::util::*;
use std::error::Error;
use std::io::{Cursor, Read};

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
	for name_index in 0..name_count {
		pos.read_exact(&mut name_buffer)?;
		names.push(String::from_utf8(lump_name(&name_buffer))?);
	}
	Ok(names)
}

pub fn read_texturex<'a>(wad: &'a wad::DoomWad, list: &'a wad::DoomWadLump, pnames: &wad::DoomWadLump) ->
	Result<TextureDefinitions<'a>, Box<dyn Error>>
{
	let patches = read_pnames(pnames)?;
	let mut pos: usize = 0;
	let mut num_buffer: [u8; 4] = [0; 4];
	let count: usize = {
		util::copy_into_array(&mut num_buffer, &list.data, pos);
		i32::from_le_bytes(num_buffer) as usize
	};
	let mut tex_def_pos: Vec<usize> = Vec::with_capacity(count);
	for _texture_index in 0..count {
		util::copy_into_array(&mut num_buffer, &list.data, pos);
		tex_def_pos.push(u32::from_le_bytes(num_buffer) as usize);
	}
	let mut name_buffer: [u8; 8] = [0; 8];
	let mut short_buffer: [u8; 2] = [0; 2];
	let mut defs = TextureDefinitions {
		textures: Vec::with_capacity(count),
		lump: list,
		// by_name: HashMap::new(),
	};
	for texture_pos in tex_def_pos.into_iter() {
		pos = texture_pos;
		pos = util::copy_into_array(&mut name_buffer, &list.data, pos);
		let name = String::from_utf8(lump_name(&name_buffer))?;
		pos = util::copy_into_array(&mut num_buffer, &list.data, pos);
		let flags = i32::from_le_bytes(num_buffer);
		pos = util::copy_into_array(&mut short_buffer, &list.data, pos);
		let width = u16::from_le_bytes(short_buffer);
		pos = util::copy_into_array(&mut short_buffer, &list.data, pos);
		let height = u16::from_le_bytes(short_buffer);
		pos += 4; // skip columndirectory
		pos = util::copy_into_array(&mut short_buffer, &list.data, pos);
		let patch_count = u16::from_le_bytes(short_buffer);
		let mut texture = Texture {
			name: name.clone(),
			flags: flags,
			width: width,
			height: height,
			patches: Vec::new(),
		};
		for _patch_index in 0..patch_count {
			pos = util::copy_into_array(&mut short_buffer, &list.data, pos);
			let x = i16::from_le_bytes(short_buffer);
			pos = util::copy_into_array(&mut short_buffer, &list.data, pos);
			let y = i16::from_le_bytes(short_buffer);
			pos = util::copy_into_array(&mut short_buffer, &list.data, pos);
			let patch_name = patches[u16::from_le_bytes(short_buffer) as usize].clone();
			pos = util::copy_into_array(&mut num_buffer, &list.data, pos);
			let flags = i32::from_le_bytes(num_buffer);
			let patch = TexturePatch {
				patch: patch_name,
				x: x,
				y: y, 
				flags: flags
			};
			texture.patches.push(patch);
		}
		defs.textures.push(texture);
	}
	Ok(defs)
}

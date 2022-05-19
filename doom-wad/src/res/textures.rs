// TEXTURE1, TEXTURE2, and PNAMES
use crate::wad::{self, DoomWad, DoomWadLump};
use crate::wad::util::*;
use crate::res::{Image, ImageFormat, ToImage};
use std::error::Error;
use std::io::{Cursor, Read, Seek, SeekFrom};

use super::DoomPicture;

pub struct TexturePatch<'a> {
	patch: String,
	x: i16, // X and Y offsets
	y: i16,
	flags: i32,
	lump: Option<&'a DoomPicture<'a>>,
}
pub struct Texture<'a> {
	name: String,
	flags: i32,
	width: u16,
	height: u16,
	patches: Vec<TexturePatch<'a>>,
}
pub struct TextureDefinitions<'a> {
	textures: Vec<Texture<'a>>,
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

pub fn read_texturex<'a>(list: &'a DoomWadLump, pnames: &DoomWadLump, wad: &'a DoomWad) ->
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
	let tex_def_pos: Vec<u64> = (0..count)
	.map(|_| -> Result<u64, Box<dyn Error>> {
		pos.read_exact(&mut num_buffer)?;
		Ok(u32::from_le_bytes(num_buffer) as u64)
	}).collect::<Result<Vec<u64>, Box<dyn Error>>>()?;
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
		defs.textures.push(Texture {
			name: name.clone(),
			flags: flags,
			width: width,
			height: height,
			patches: (0..patch_count).map(|_| -> Result<TexturePatch, Box<dyn Error>> {
				pos.read_exact(&mut short_buffer)?;
				let x = i16::from_le_bytes(short_buffer);
				pos.read_exact(&mut short_buffer)?;
				let y = i16::from_le_bytes(short_buffer);
				pos.read_exact(&mut short_buffer)?;
				let patch_name = patches[u16::from_le_bytes(short_buffer) as usize].clone();
				pos.read_exact(&mut num_buffer)?;
				let flags = i32::from_le_bytes(num_buffer);
				Ok(TexturePatch {
					patch: patch_name,
					x: x,
					y: y, 
					flags: flags,
					lump: None
				})
			}).collect::<Result<Vec<TexturePatch>, Box<dyn Error>>>()?
		});
		Ok(())
	})?;
	Ok(defs)
}

impl<'a> ToImage for Texture<'a> {
	fn to_image(&self) -> Image {
		let mut image = Image::new(self.width as usize, self.height as usize, ImageFormat::IndexedAlpha);
		self.patches.iter().for_each(|pa| {
			match pa.lump {
				Some(lump) => {
					let patch_image = lump.to_image();
					let blit_res = image.blit(&patch_image, pa.x as i32, pa.y as i32);
					if let Err(e) = blit_res {
						eprintln!("{}", e);
					}
				},
				None => (),
			};
		});
		image
	}
}

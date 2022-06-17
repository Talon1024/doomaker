// TEXTURE1, TEXTURE2, and PNAMES
use crate::wad::{self, DoomWadLump, LumpName, GetLump};
use crate::res::{Image, ImageFormat, ToImage};
use std::{collections::HashMap, error::Error};
use std::io::{Cursor, Read, Seek, SeekFrom};
use ahash::RandomState;
use derive_deref::*;
use super::DoomPicture;

#[derive(Debug, Clone)]
pub struct TexturePatch<'a> {
	patch: LumpName,
	x: i16, // X and Y offsets
	y: i16,
	flags: i32,
	lump: Option<DoomPicture<'a>>,
}

#[derive(Debug, Clone)]
pub struct Texture<'a> {
	name: LumpName,
	flags: i32,
	width: u16,
	height: u16,
	patches: Vec<TexturePatch<'a>>,
}

#[derive(Debug, Clone)]
pub struct TextureDefinitions<'a> {
	textures: Vec<Texture<'a>>,
}

impl<'a> TextureDefinitions<'a> {
	pub fn tex_map(&'a self) -> TextureDefinitionsMap<'a> {
		let mut map = TextureDefinitionsMap::default();
		self.textures.iter().for_each(|t| {
			map.insert(t.name, &t);
		});
		map
	}
}

#[derive(Default, Deref, Debug)]
pub struct TextureDefinitionsLumps<'a>(pub(crate) Vec<TextureDefinitions<'a>>);

impl<'a> TextureDefinitionsLumps<'a> {
	pub fn tex_map(&'a self) -> TextureDefinitionsMap<'a> {
		self.0.iter().map(TextureDefinitions::tex_map).reduce(|mut a, b| {
			a.extend(b); a
		}).unwrap_or_default()
	}
}

// https://users.rust-lang.org/t/hashmap-of-a-vector-of-objects/29220
// My solution is to add a method to the referred type which creates a HashMap
// of references to the data in the referree.
pub type TextureDefinitionsMap<'a> =
	HashMap<LumpName, &'a Texture<'a>, RandomState>;

fn read_pnames(pnames: &wad::DoomWadLump) ->
	Result<Vec<LumpName>, Box<dyn Error>>
{
	let mut num_buffer: [u8; 4] = [0; 4];
	let mut name_buffer: [u8; 8] = [0; 8];
	let mut pos = Cursor::new(&pnames.data);
	let name_count: usize = {
		pos.read_exact(&mut num_buffer)?;
		u32::from_le_bytes(num_buffer) as usize
	};
	(0..name_count).map(|_| {
		pos.read_exact(&mut name_buffer)?;
		LumpName::try_from(&name_buffer).map_err(Box::from)
	}).collect()
}

pub fn read_texturex<'a>(
	list: &'a DoomWadLump, pnames: &'a DoomWadLump, wad: &'a (dyn GetLump<'a>)) ->
	Result<TextureDefinitions<'a>, Box<dyn Error>>
{
	let patches = read_pnames(pnames)?;
	let mut pos = Cursor::new(&list.data);
	let mut name_buffer: [u8; 8] = [0; 8];
	let mut num_buffer: [u8; 4] = [0; 4];
	let mut short_buffer: [u8; 2] = [0; 2];
	let count: usize = {
		pos.read_exact(&mut num_buffer)?;
		u32::from_le_bytes(num_buffer) as usize
	};
	let tex_def_pos: Vec<u64> = (0..count)
	.map(|_| -> Result<u64, Box<dyn Error>> {
		pos.read_exact(&mut num_buffer)?;
		Ok(u32::from_le_bytes(num_buffer) as u64)
	}).collect::<Result<Vec<u64>, Box<dyn Error>>>()?;
	let mut defs = TextureDefinitions {
		textures: Vec::with_capacity(count),
	};
	tex_def_pos.into_iter().try_for_each(|offset| -> Result<(), Box<dyn Error>> {
		pos.seek(SeekFrom::Start(offset))?;
		// Name (8 bytes)
		pos.read_exact(&mut name_buffer)?; // name
		let name = LumpName::try_from(&name_buffer)?;
		// Flags (4 bytes)
		pos.read_exact(&mut num_buffer)?; // masked
		let flags = i32::from_le_bytes(num_buffer);
		pos.read_exact(&mut short_buffer)?; // width
		let width = u16::from_le_bytes(short_buffer);
		pos.read_exact(&mut short_buffer)?; // height
		let height = u16::from_le_bytes(short_buffer);
		pos.seek(SeekFrom::Current(4))?; // skip columndirectory
		pos.read_exact(&mut short_buffer)?; // patchcount
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
				let pindex = u16::from_le_bytes(short_buffer);
				let patch_name = patches[pindex as usize];
				pos.read_exact(&mut num_buffer)?;
				// Two unused 16-bit integers
				let flags = i32::from_le_bytes(num_buffer);
				Ok(TexturePatch {
					patch: patch_name,
					x: x,
					y: y, 
					flags: flags,
					lump: wad.get_lump(patch_name).map(DoomPicture::from)
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
			match &pa.lump {
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


#[cfg(test)]
mod tests {

	use crate::wad::{DoomWadType, DoomWad};
	use super::*;

	#[test]
	fn reads_texturex() -> Result<(), Box<dyn Error>> {
		let texture1_name = LumpName(*b"TEXTURE1");
		let pnames_name = LumpName(*b"PNAMES\0\0");
		let wad = DoomWad {
			wtype: DoomWadType::PWAD,
			lumps: vec![DoomWadLump {
				name: texture1_name,
				data: Vec::from(*include_bytes!("../../tests/data/TEXTURE1.lmp"))
			}, DoomWadLump {
				name: pnames_name,
				data: Vec::from(*include_bytes!("../../tests/data/PNAMES.lmp"))
			}]
		};
		let texture_lump = wad.get_lump(texture1_name).ok_or("No TEXTURE1!")?;
		let pnames_lump = wad.get_lump(pnames_name).ok_or("No PNAMES!")?;
		let texdefs = read_texturex(texture_lump, pnames_lump, &wad)?;
		assert_eq!(texdefs.textures.len(), 4);
		assert_eq!(texdefs.textures[0].name, LumpName(*b"S3DUMMY\0"));
		Ok(())
	}
}

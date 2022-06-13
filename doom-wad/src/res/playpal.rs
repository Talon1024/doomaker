// Palette
// All graphics in a Doom WAD are stored as palette indices rather than RGB or
// RGBA samples

use crate::wad::DoomWadLump;
use crate::wad::lump_name::LumpName;
use std::error::Error;
use std::fmt;
use crate::res::{Image, ToImage, ImageFormat, ImageDimension};

const BYTES_PER_PALETTE: usize = 768; // 256 colors * RGB channels

type Palette = [u8; BYTES_PER_PALETTE];
// book/ch10-03-lifetime-syntax.html#lifetime-annotations-in-struct-definitions
pub struct PaletteCollection<'a> {
	lump: &'a DoomWadLump
}

// rust-by-example/scope/lifetime/trait.html
impl<'a> From<&'a DoomWadLump> for PaletteCollection<'a> {
	fn from(lump: &'a DoomWadLump) -> PaletteCollection<'a> {
		PaletteCollection { lump: lump }
	}
}

#[derive(Debug)]
enum PaletteError {
	NoPalettes
}
impl fmt::Display for PaletteError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			PaletteError::NoPalettes => {
				write!(f, "Palette collection has no palettes")?;
			}
		}
		Ok(())
	}
}
impl Error for PaletteError {}

impl PaletteCollection<'_> {
	pub fn count(&self) -> usize {
		self.lump.data.len() / BYTES_PER_PALETTE
	}
	pub fn get(&self, index: usize) -> Result<Palette, Box<dyn Error>> {
		let count = self.count();
		if count == 0 {
			return Err(Box::new(PaletteError::NoPalettes));
		}
		let start: usize = index * BYTES_PER_PALETTE;
		let end: usize = start + BYTES_PER_PALETTE;
		Ok(Palette::try_from(&self.lump.data[start..end])?)
	}
}

impl<'a> ToImage for PaletteCollection<'a> {
	fn to_image(&self) -> Image {
		let rows = self.count() as ImageDimension;
		let rgb: Vec<u8> = self.lump.data.clone();
		Image {
			width: 256,
			height: rows,
			data: rgb,
			x: 0, y: 0,
			format: ImageFormat::RGB
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn imports_properly() {
		let playpal = DoomWadLump {
			name: LumpName::try_from("PLAYPAL").unwrap(),
			data: Vec::from(include_bytes!("../../tests/data/PLAYPAL.pal").as_slice()),
		};
		let palcol = PaletteCollection {lump: &playpal};
		assert_eq!(palcol.count(), 14);
	}

	#[test]
	fn can_get_palettes() -> Result<(), Box<dyn Error>> {
		let playpal = DoomWadLump {
			name: LumpName::try_from("PLAYPAL").unwrap(),
			data: Vec::from(include_bytes!("../../tests/data/PLAYPAL.pal").as_slice()),
		};
		let palcol = PaletteCollection {lump: &playpal};

		palcol.get(0)?;
		palcol.get(1)?;
		palcol.get(13)?;
		Ok(())
	}

	#[test]
	#[should_panic]
	fn bad_palette_index() -> () {
		let playpal = DoomWadLump {
			name: LumpName::try_from("PLAYPAL").unwrap(),
			data: Vec::from(include_bytes!("../../tests/data/PLAYPAL.pal").as_slice()),
		};
		let palcol = PaletteCollection {lump: &playpal};

		// The palette collection has only 14 palettes, starting at 0. This
		// tries (and fails) to get palette #15.
		palcol.get(14).unwrap();
		()
	}
}

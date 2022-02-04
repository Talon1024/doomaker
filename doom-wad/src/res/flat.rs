// Flats are 64x64 graphics stored as indexed samples in row-major order.
use crate::wad::DoomWadLump;
use crate::res::{ToImage, Image, ImageFormat};

const MINIMUM_SIZE: usize = 64;
// const MINIMUM_BYTES: usize = 4096; // 64 * 64

pub struct FlatImage<'a> {
	lump: &'a DoomWadLump
}

impl<'a> FlatImage<'a> {
	pub fn height(&self) -> usize {
		let l = self.lump.data.len();
		let mut h = l / MINIMUM_SIZE;
		if l % MINIMUM_SIZE > 0 {
			h += 1;
		}
		h
	}
	pub fn width(&self) -> usize {
		MINIMUM_SIZE.min(self.lump.data.len())
	}
}

impl<'a> ToImage for FlatImage<'a> {
	fn to_image(&self) -> Image {
		let height = self.height();
		let width = self.width();
		let len = self.lump.data.len();

		/*
		let mut data: Vec<u8> = [0u8]
			.iter().cycle().take(width * height).copied().collect();
		*/
		let mut data = vec![0; width * height];
		let _ = &data[..len].copy_from_slice(&self.lump.data);
		Image {
			width, height, data, x: 0, y: 0,
			format: ImageFormat::Indexed
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn converts_flats_properly() {
		let flat_lump = DoomWadLump {
			name: String::from(""),
			data: Vec::from(include_bytes!("../../tests/data/TLITE6_5.flt").as_slice())
		};
		let expected = Image {
			width: 64,
			height: 64,
			x: 0,
			y: 0,
			data: Vec::from(include_bytes!("../../tests/data/TLITE6_5.flt").as_slice()),
			format: ImageFormat::Indexed
		};

		let flat_image = FlatImage {lump: &flat_lump};
		let flat_image = flat_image.to_image();

		assert_eq!(flat_image.width, expected.width);
		assert_eq!(flat_image.height, expected.height);
		assert_eq!(flat_image.format, expected.format);
		assert_eq!(flat_image.data.len(), expected.data.len());

		assert!(flat_lump.data.iter().eq(expected.data.iter()));
	}

	#[test]
	// Heretic's F_SKY1 lump is only 4 bytes long
	fn converts_heretic_sky() {
		let flat_lump = DoomWadLump {
			name: String::from("F_SKY1"),
			data: Vec::<u8>::from([83, 75, 89, 10])
		};
		let expected = Image {
			width: 4,
			height: 1,
			x: 0,
			y: 0,
			data: Vec::<u8>::from([83, 75, 89, 10]),
			format: ImageFormat::Indexed
		};

		let flat_image = FlatImage {lump: &flat_lump};
		let flat_image = flat_image.to_image();

		assert_eq!(flat_image.width, expected.width);
		assert_eq!(flat_image.height, expected.height);
		assert_eq!(flat_image.format, expected.format);
		assert_eq!(flat_image.data.len(), expected.data.len());

		assert!(flat_lump.data.iter().eq(expected.data.iter()));
	}

	#[test]
	// How about a flat with only 1.5 rows?
	fn converts_incomplete_flat() {
		let flat_lump = DoomWadLump {
			name: String::from("INCOMPLE"),
			data: Vec::from(include_bytes!("../../tests/data/INCOMPLE.flt").as_slice())
		};
		let expected = Image {
			width: 64,
			height: 2,
			x: 0,
			y: 0,
			data: {
				let mut v = Vec::from(include_bytes!("../../tests/data/INCOMPLE.flt").as_slice());
				let vl = v.len();
				v.extend(vec![0; 64 * 2 - vl]); // v should be 128 bytes long at this point
				v
			},
			format: ImageFormat::Indexed
		};

		let flat_image = FlatImage {lump: &flat_lump};
		let flat_image = flat_image.to_image();

		assert_eq!(flat_image.width, expected.width);
		assert_eq!(flat_image.height, expected.height);
		assert_eq!(flat_image.format, expected.format);
		assert_eq!(flat_image.data.len(), expected.data.len());

		assert!(flat_image.data.iter().eq(expected.data.iter()));
	}
}

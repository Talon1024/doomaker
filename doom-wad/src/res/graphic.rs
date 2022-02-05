use crate::wad::DoomWadLump;
use crate::res::{self, ToImage, Image, ImageFormat};
use std::io::{Read, Cursor, Seek, SeekFrom};
use std::error::Error;

pub struct DoomPicture<'a> {
	lump: &'a DoomWadLump
}

struct DoomPicturePost {
	column: usize,
	top_delta: u8,
	pixels: Vec<u8>
}

impl<'a> ToImage for DoomPicture<'a> {
	fn to_image(&self) -> Image {
		let mut short_buffer: [u8; 2] = [0; 2];
		let mut long_buffer: [u8; 4] = [0; 4];
		let mut pos = Cursor::new(&self.lump.data);
		let bad_image = Image {
			width: 0, height: 0, x: 0, y: 0,
			format: ImageFormat::Indexed, data: Vec::new()
		};
		let width = {
			if pos.read_exact(&mut short_buffer).is_err() {
				return bad_image;
			}
			u16::from_le_bytes(short_buffer)
		} as usize;
		let height = {
			if pos.read_exact(&mut short_buffer).is_err() {
				return bad_image;
			}
			u16::from_le_bytes(short_buffer)
		} as usize;
		let x = {
			if pos.read_exact(&mut short_buffer).is_err() {
				return bad_image;
			}
			i16::from_le_bytes(short_buffer)
		};
		let y = {
			if pos.read_exact(&mut short_buffer).is_err() {
				return bad_image;
			}
			i16::from_le_bytes(short_buffer)
		};
		let column_offsets: Result<Vec<usize>, Box<dyn Error>> = (0usize..width).map(|_| {
			pos.read_exact(&mut long_buffer)?;
			Ok(u32::from_le_bytes(long_buffer) as usize)
		}).collect();
		if column_offsets.is_err() {
			return bad_image;
		}
		let column_offsets = column_offsets.unwrap();

		let mut data = vec![0u8; width * height];
		let mut alpha = vec![0u8; width * height];
		let mut opaque_pixels: usize = 0;
		// let mut col_heights = vec![0usize; width];
		// let mut pixel_count = vec![0usize; 255];

		column_offsets.iter()
		.enumerate().map(|(column, &offset)| {
			if pos.seek(SeekFrom::Start(offset as u64)).is_err() {
				return Vec::new();
			}
			let mut cur_byte: [u8; 1] = [0];
			let mut posts: Vec<DoomPicturePost> = Vec::new();
			loop {

				if pos.read_exact(&mut cur_byte).is_err() {
					return posts;
				}
				let top_delta = cur_byte[0];

				if top_delta == 255 {
					break
				}

				if pos.read_exact(&mut cur_byte).is_err() {
					return posts;
				}
				let length = cur_byte[0];

				if pos.seek(SeekFrom::Current(1)).is_err() {
					// Unused padding byte
					return posts;
				}

				let mut pixels = vec![0u8; length as usize];
				if pos.read_exact(&mut pixels).is_err() {
					return posts;
				}

				if pos.seek(SeekFrom::Current(1)).is_err() {
					// Unused padding byte
					return posts;
				}
				posts.push(DoomPicturePost {
					column, top_delta, pixels
				});
			}
			posts
		}).for_each(|col_posts| {
			let mut coly = 0usize;
			col_posts.iter().for_each(|post| {
				let top_delta = post.top_delta as usize;
				let y = if top_delta <= coly {
					coly + top_delta
				} else {
					top_delta
				};
				coly = y;
				post.pixels.iter().enumerate()
				.for_each(|(pixpos, &pixel)| {
					if let Some(bp) = res::xy_to_bufpos(
							post.column, y + pixpos, width, height, 1) {
						// pixel_count[pixel as usize] += 1;
						data[bp] = pixel; // Index
						alpha[bp] = 255; // Alpha
						opaque_pixels += 1;
					}
				});
			});
		});
		let format = {
			if opaque_pixels == width * height {
				ImageFormat::Indexed
			} else {
				ImageFormat::IndexedAlpha
			}
		};
		if format == ImageFormat::IndexedAlpha {
			let mut pixels = vec![0u8; width * height * format.channels()];
			pixels.chunks_mut(2).zip(data).zip(alpha)
			.for_each(|((chunk, index), alpha)| {
				chunk[0] = index; chunk[1] = alpha;
			});
			Image {
				width, height, data: pixels, format,
				x: x as i32, y: y as i32
			}
		} else {
			Image {
				width, height, data, format,
				x: x as i32, y: y as i32
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn converts_opaque_patches_correctly() {
		let patch_lump = DoomWadLump {
			name: String::from("MOSSBRK8"),
			data: Vec::from(include_bytes!("../../tests/data/MOSSBRK8.lmp").as_slice())
		};
		let expected = Image {
			width: 128,
			height: 128,
			x: 64,
			y: 123,
			data: Vec::from(include_bytes!("../../tests/data/MOSSBRK8.raw").as_slice()),
			format: ImageFormat::Indexed
		};

		let picture = DoomPicture {lump: &patch_lump};
		let image = picture.to_image();

		assert_eq!(image.width, expected.width);
		assert_eq!(image.height, expected.height);
		assert_eq!(image.x, expected.x);
		assert_eq!(image.y, expected.y);
		assert_eq!(image.format, expected.format);
		assert_eq!(image.data.len(), expected.data.len());

		assert!(image.data.iter().eq(expected.data.iter()));
	}

	#[test]
	fn converts_transparent_patches_correctly() {
		let patch_lump = DoomWadLump {
			name: String::from("GRATE"),
			data: Vec::from(include_bytes!("../../tests/data/GRATE.lmp").as_slice())
		};
		let expected = Image {
			width: 128,
			height: 128,
			x: 64,
			y: 123,
			data: Vec::from(include_bytes!("../../tests/data/GRATE.raw").as_slice()),
			format: ImageFormat::IndexedAlpha
		};

		let picture = DoomPicture {lump: &patch_lump};
		let image = picture.to_image();

		assert_eq!(image.width, expected.width);
		assert_eq!(image.height, expected.height);
		assert_eq!(image.x, expected.x);
		assert_eq!(image.y, expected.y);
		assert_eq!(image.format, expected.format);
		assert_eq!(image.data.len(), expected.data.len());

		assert!(image.data.iter().eq(expected.data.iter()));
	}

	#[test]
	fn converts_tall_patches_correctly() {
		let patch_lump = DoomWadLump {
			name: String::from("SHTGC0"),
			data: Vec::from(include_bytes!("../../tests/data/SHTGC0.lmp").as_slice())
		};
		let expected = Image {
			width: 98,
			height: 146,
			x: -27,
			y: -22,
			data: Vec::from(include_bytes!("../../tests/data/SHTGC0.raw").as_slice()),
			format: ImageFormat::IndexedAlpha
		};

		let picture = DoomPicture {lump: &patch_lump};
		let image = picture.to_image();

		assert_eq!(image.width, expected.width);
		assert_eq!(image.height, expected.height);
		assert_eq!(image.x, expected.x);
		assert_eq!(image.y, expected.y);
		assert_eq!(image.format, expected.format);
		assert_eq!(image.data.len(), expected.data.len());

		assert!(image.data.iter().eq(expected.data.iter()));
	}

	#[test]
	fn converts_deepsea_tall_patches_correctly() {
		let patch_lump = DoomWadLump {
			name: String::from("CYBRE1"),
			data: Vec::from(include_bytes!("../../tests/data/CYBRE1.lmp").as_slice())
		};
		let expected = Image {
			width: 277,
			height: 335,
			x: 138,
			y: 331,
			data: Vec::from(include_bytes!("../../tests/data/CYBRE1.raw").as_slice()),
			format: ImageFormat::IndexedAlpha
		};

		let picture = DoomPicture {lump: &patch_lump};
		let image = picture.to_image();

		assert_eq!(image.width, expected.width);
		assert_eq!(image.height, expected.height);
		assert_eq!(image.x, expected.x);
		assert_eq!(image.y, expected.y);
		assert_eq!(image.format, expected.format);
		assert_eq!(image.data.len(), expected.data.len());

		assert!(image.data.iter().eq(expected.data.iter()));
	}
}

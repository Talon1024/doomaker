use crate::wad::DoomWadLump;
use crate::res::{self, ToImage, Image, ImageFormat, ImageDimension};
use std::io::{Read, Cursor, Seek, SeekFrom};
use std::error::Error;

#[derive(Debug, Clone)]
pub struct DoomPicture<'a> {
	lump: &'a DoomWadLump
}

impl<'a> From<&'a DoomWadLump> for DoomPicture<'a> {
	fn from(lump: &'a DoomWadLump) -> DoomPicture<'a> {
		DoomPicture { lump: lump }
	}
}

#[cfg(feature = "png")]
impl<'a> DoomPicture<'a> {
	const PNG_HEADER: [u8; 8] = *b"\x89PNG\r\n\x1A\n";
	fn read_png(&self) -> Option<Image> {
		let mut png_head_buf: [u8; 8] = [0; 8];
		let mut pos = Cursor::new(&self.lump.data);
		if pos.read_exact(&mut png_head_buf).is_err() {
			return None;
		}
		if png_head_buf == Self::PNG_HEADER {
			pos.set_position(0);
			let transform = png::Transformations::normalize_to_color8();
			let mut decoder = png::Decoder::new(pos);
			decoder.set_transformations(transform);
			let mut reader = decoder.read_info().ok()?;
			let mut data = vec![0; reader.output_buffer_size()];
			reader.next_frame(&mut data).ok()?;
			let png::Info {width, height, ..} = *reader.info();
			let (color_type, _bit_depth) = reader.output_color_type();
			let format = match color_type {
				png::ColorType::Rgb => Some(ImageFormat::RGB),
				png::ColorType::Rgba => Some(ImageFormat::RGBA),
				_ => None
			}?;
			Some(Image {
				width: width as usize,
				height: height as usize,
				data,
				x: 0, // Requires handling unknown chunks like grAb. The `png`
				y: 0, // crate currently does not support unknown chunks
				format,
			})
		} else {
			None
		}
	}
}


impl<'a> ToImage for DoomPicture<'a> {
	fn to_image(&self) -> Image {

		// TODO: Format detection and processing
		struct DoomPicturePost {
			column: ImageDimension,
			top_delta: u8,
			pixels: Vec<u8>
		}

		let mut short_buffer: [u8; 2] = [0; 2];
		let mut long_buffer: [u8; 4] = [0; 4];
		let mut pos = Cursor::new(&self.lump.data);

		// In case the patch is bad
		let bad_image = Image {
			width: 0, height: 0, x: 0, y: 0,
			format: ImageFormat::Indexed, data: Vec::new()
		};

		#[cfg(feature = "png")]
		if let Some(image) = self.read_png() {
			return image;
		}

		let width = {
			if pos.read_exact(&mut short_buffer).is_err() {
				return bad_image;
			}
			u16::from_le_bytes(short_buffer)
		} as ImageDimension;
		let height = {
			if pos.read_exact(&mut short_buffer).is_err() {
				return bad_image;
			}
			u16::from_le_bytes(short_buffer)
		} as ImageDimension;
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

		// Column offsets are relative to the start of the lump
		let column_offsets: Result<Vec<usize>, Box<dyn Error>> = (0..width).map(|_| {
			pos.read_exact(&mut long_buffer)?;
			Ok(u32::from_le_bytes(long_buffer) as usize)
		}).collect();
		if column_offsets.is_err() {
			return bad_image;
		}
		let column_offsets = column_offsets.unwrap();

		let image_pixels = (width * height) as usize;
		let mut data = vec![0u8; image_pixels];
		let mut alpha = vec![0u8; image_pixels];
		let mut opaque_pixels: usize = 0;

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
					column: column as ImageDimension, top_delta, pixels
				});
			}
			posts
		}).for_each(|col_posts| {
			let mut coly = 0 as ImageDimension;
			col_posts.iter().for_each(|post| {
				let top_delta = post.top_delta as ImageDimension;
				let y = if top_delta <= coly {
					coly + top_delta
				} else {
					top_delta
				};
				coly = y;
				post.pixels.iter().enumerate()
				.for_each(|(pixpos, &pixel)| {
					let pixpos = pixpos as ImageDimension;
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
			if opaque_pixels == image_pixels {
				ImageFormat::Indexed  // Fully opaque
			} else {
				ImageFormat::IndexedAlpha
			}
		};
		// Partially or fully transparent
		if format == ImageFormat::IndexedAlpha {
			// 2 channels - index and alpha
			let channels = format.channels() as usize;
			let mut pixels = vec![0u8; image_pixels * channels];
			pixels.chunks_exact_mut(channels).zip(data).zip(alpha)
			.for_each(|((chunk, index), alpha)| {
				chunk[0] = index; chunk[1] = alpha;
			});
			Image {
				width, height, data: pixels, format,
				x: x as i32, y: y as i32
			}
		} else {  // Fully opaque
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
	use crate::wad::LumpName;

	#[test]
	fn converts_opaque_patches_correctly() {
		let patch_lump = DoomWadLump {
			name: LumpName::try_from("MOSSBRK8").unwrap(),
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
			name: LumpName::try_from("GRATE").unwrap(),
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
			name: LumpName::try_from("SHTGC0").unwrap(),
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
			name: LumpName::try_from("CYBRE1").unwrap(),
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

	#[test]
	fn converts_tswgb0_correctly() {
		let patch_lump = DoomWadLump {
			name: LumpName::try_from("TSWGB0").unwrap(),
			data: Vec::from(include_bytes!("../../tests/data/TSWGB0.lmp").as_slice())
		};
		let expected = Image {
			width: 179,
			height: 333,
			x: -249,
			y: 155,
			data: Vec::from(include_bytes!("../../tests/data/TSWGB0.raw").as_slice()),
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

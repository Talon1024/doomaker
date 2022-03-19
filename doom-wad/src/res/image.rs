use std::error::Error;
use std::fmt::{Debug, Display};

mod blending;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ImageFormat {
	RGB,
	RGBA,
	Indexed,
	IndexedAlpha
}

impl ImageFormat {
	pub fn channels(&self) -> ImageDimension {
		match self {
			ImageFormat::RGB => 3,
			ImageFormat::RGBA => 4,
			ImageFormat::Indexed => 1,
			ImageFormat::IndexedAlpha => 2,
		}
	}
	pub fn alpha(&self) -> Option<usize> {
		match self {
			ImageFormat::RGB => None,
			ImageFormat::RGBA => Some(3),
			ImageFormat::Indexed => None,
			ImageFormat::IndexedAlpha => Some(1),
		}
	}
	pub fn equivalent_alpha(&self, other: ImageFormat) -> bool {
		match self {
			ImageFormat::RGB => other == ImageFormat::RGBA,
			ImageFormat::RGBA => other == ImageFormat::RGBA,
			ImageFormat::Indexed => other == ImageFormat::IndexedAlpha,
			ImageFormat::IndexedAlpha => other == ImageFormat::IndexedAlpha
		}
	}
}

impl Display for ImageFormat {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		match self {
			ImageFormat::RGB => write!(f, "RGB"),
			ImageFormat::RGBA => write!(f, "RGBA"),
			ImageFormat::Indexed => write!(f, "Indexed"),
			ImageFormat::IndexedAlpha => write!(f, "IndexedAlpha"),
		}
	}
}

#[derive(Debug, PartialEq)]
pub enum ImageError {
	/// The user is trying to convert the image into a format which it cannot
	/// be converted. For example, they are trying to convert an RGB image into
	/// an indexed image. This will not work.
	IncompatibleFormat { my: ImageFormat, your: ImageFormat },
	/// The user is trying to blit an image in a different format onto this one.
	DifferentFormat,
	OutOfBounds { x: ImageDimension, y: ImageDimension }
}

impl Display for ImageError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		match self {
			ImageError::IncompatibleFormat{my, your} => write!(f, "This image,which is in {} format, cannot be converted to {} format.", my, your),
			ImageError::DifferentFormat => write!(f, "The image formats do not match!"),
			ImageError::OutOfBounds{x, y} => write!(f, "({} {}) is outside of this image's boundaries!", x, y)
		}
	}
}

impl Error for ImageError{}

pub type ImageDimension = u32;

pub struct Image {
	pub width: ImageDimension,
	pub height: ImageDimension,
	pub data: Vec<u8>,
	pub x: i32,
	pub y: i32,
	pub format: ImageFormat
}

impl Image {
	pub fn new(width: ImageDimension, height: ImageDimension, format: ImageFormat) -> Image {
		let channels = format.channels();
		let image_size = (width * height * channels) as usize;
		Image {
			width, height, x: 0, y: 0, format,
			data: vec![0u8; image_size]
		}
	}
	pub fn blit(&mut self, other: &Image, x: ImageDimension, y: ImageDimension) -> Result<(), Box<dyn Error>> {
		if x > self.width || y > self.height {
			return Err(Box::new(ImageError::OutOfBounds{x, y}))
		}
		if self.format.equivalent_alpha(other.format) {
			return Err(Box::new(ImageError::DifferentFormat));
		}
		let channels = self.format.channels();
		let mut data_slice = {
			// For the mutable slice of the data, I need the start of row y,
			// and the end of row y + other.height
			let slice_start = (y * self.width * channels) as usize;
			let slice_end = (slice_start +
				(other.height * self.width * channels) as usize)
				.min(self.data.len());
			&mut self.data[slice_start..slice_end]
		};
		let mut other_row: ImageDimension = 0;
		data_slice.chunks_exact_mut(channels as usize)
			.zip((0..other.width).cycle())
			.for_each(|(pixel, other_col)| {
				let self_col = other_col + x;
				let self_row = other_row + y;
				if let Some(_) = xy_to_bufpos(self_col, self_row, self.width, self.height, channels) {
					let other_slice = {
						let slice_start = ((other_row * other.width + other_col) * channels) as usize;
						let slice_end = slice_start + channels as usize;
						&other.data[slice_start..slice_end]
					};
					if let Some(index) = self.format.alpha() {
						if other_slice[index] != 0 {
							pixel.copy_from_slice(other_slice);
						}
					} else {
						pixel.copy_from_slice(other_slice);
					}
				}
				if other_col == other.width - 1 {
					other_row += 1;
				}
			});
		Ok(())
	}
	pub fn convert_to(&mut self, format: ImageFormat) -> Result<(), Box<dyn Error>> {
		if self.format == format {
			return Ok(());
		}
		// Allow indexed images to be converted to IndexedAlpha
		if self.format == ImageFormat::Indexed {

		}
		Ok(())
	}
}

pub trait ToImage {
	fn to_image(&self) -> Image;
}

impl Image {
	// Generate grayscale palette
	pub fn grayscale_palette() -> [u8; 768] {
		let mut component = 0;
		let mut color: u8 = 0;
		[0; 768].map(|_| {
			if component == 3 {
				color += 1;
				component = 0;
			}
			component += 1;
			color
		})
	}
	pub fn to_rgb(&mut self, pal: Option<[u8; 768]>) {
		let pal = pal.unwrap_or(Image::grayscale_palette());
		match self.format {
			ImageFormat::RGB => (),
			ImageFormat::RGBA => {

			},
			ImageFormat::Indexed => {

			},
			ImageFormat::IndexedAlpha => {

			}
		}
	}
	/*
	pub fn clone_to_rgb(&self, pal: Option<[u8; 768]>) -> Image {
		let image = self.clone();
		image.to_rgb(pal);
		image
	}
	*/
}

pub fn xy_to_bufpos(x: ImageDimension, y: ImageDimension, w: ImageDimension, h: ImageDimension, channels: ImageDimension) -> Option<usize> {
	if x >= w {
		// No need to check y >= h because if it is, the calculated buffer
		// position will be greater than the calculated image size
		return None; 
	}
	let size = w * h * channels;
	let pos = y * w * channels + x * channels;
	if pos < size {
		Some(pos as usize)
	} else {
		None
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn xy_to_bufpos_works() {
		// 1 channel
		assert_eq!(xy_to_bufpos(4, 7, 128, 128, 1), Some(900));
		assert_eq!(xy_to_bufpos(128, 7, 128, 128, 1), None);
		assert_eq!(xy_to_bufpos(4, 128, 128, 128, 1), None);
		assert_eq!(xy_to_bufpos(127, 127, 128, 128, 1), Some(16383));
		assert_eq!(xy_to_bufpos(128, 127, 128, 128, 1), None);

		// 2 channels
		assert_eq!(xy_to_bufpos(4, 7, 128, 128, 2), Some(1800));
		assert_eq!(xy_to_bufpos(128, 7, 128, 128, 2), None);
		assert_eq!(xy_to_bufpos(4, 128, 128, 128, 2), None);
		assert_eq!(xy_to_bufpos(127, 127, 128, 128, 2), Some(32766));
		assert_eq!(xy_to_bufpos(128, 127, 128, 128, 2), None);

		// 3 channels
		assert_eq!(xy_to_bufpos(4, 7, 128, 128, 3), Some(2700));
		assert_eq!(xy_to_bufpos(128, 7, 128, 128, 3), None);
		assert_eq!(xy_to_bufpos(4, 128, 128, 128, 3), None);
		assert_eq!(xy_to_bufpos(127, 127, 128, 128, 3), Some(49149));
		assert_eq!(xy_to_bufpos(128, 127, 128, 128, 3), None);

		// 4 channels
		assert_eq!(xy_to_bufpos(4, 7, 128, 128, 4), Some(3600));
		assert_eq!(xy_to_bufpos(128, 7, 128, 128, 4), None);
		assert_eq!(xy_to_bufpos(4, 128, 128, 128, 4), None);
		assert_eq!(xy_to_bufpos(127, 127, 128, 128, 4), Some(65532));
		assert_eq!(xy_to_bufpos(128, 127, 128, 128, 4), None);
	}
}

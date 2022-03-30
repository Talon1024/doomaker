use thiserror::Error;
use std::fmt::{Debug, Display};
use std::ops::Range;

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

#[derive(Debug, PartialEq, Error)]
pub enum ImageError {
	/// The user is trying to convert the image into a format which it cannot
	/// be converted. For example, they are trying to convert an RGB image into
	/// an indexed image. This will not work.
	#[error("This image,which is in {my} format, cannot be converted to {your} format.")]
	IncompatibleFormat { my: ImageFormat, your: ImageFormat },
	/// The user is trying to blit an image in a different format onto this one.
	#[error("The image formats do not match!")]
	DifferentFormat,
	/// The target image is outside of the original image's bounds
	#[error("({x} {y}) is outside of this image's boundaries!")]
	OutOfBounds { x: i32, y: i32 }
}

pub type ImageDimension = usize;

pub struct Image {
	pub width: ImageDimension,
	pub height: ImageDimension,
	pub data: Vec<u8>,
	pub x: i32,
	pub y: i32,
	pub format: ImageFormat
}

struct BlitView {
	awidh: usize,
	aminx: usize,
	aminy: usize,
	bwidh: usize,
	bminx: usize,
	bminy: usize,
	channels: usize,
	rows: usize,
	cols: usize,
	row: usize,
}

impl Iterator for BlitView {
	type Item = (Range<usize>, Range<usize>);
	fn next(&mut self) -> Option<Self::Item> {
		if self.row < self.rows {
			// Row * width + column
			let asta = (self.aminy * self.awidh + self.aminx) * self.channels;
			let aend = asta + self.cols * self.channels;
			let bsta = (self.bminy * self.bwidh + self.bminx) * self.channels;
			let bend = bsta + self.cols * self.channels;
			Some((asta..aend, bsta..bend))
		} else {
			None
		}
	}
}

impl From<(&Image, &Image, i32, i32)> for BlitView {
	fn from(v: (&Image, &Image, i32, i32)) -> BlitView {
		BlitView {
			awidh: v.0.width,
			aminx: (v.2.max(0) as usize).min(v.0.width - 1),
			aminy: (v.3.max(0) as usize).min(v.0.height - 1),
			bwidh: v.1.width,
			bminx: (-v.2.max(0)) as usize,
			bminy: (-v.3.max(0)) as usize,
			channels: v.0.format.channels(),
			rows: ((v.3.max(0) as usize) + v.1.height).min(v.0.height),
			cols: ((v.2.max(0) as usize) + v.1.width).min(v.0.width),
			row: 0,
		}
	}
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
	pub fn blit(&mut self, other: &Image, x: i32, y: i32) -> Result<(), ImageError> {
		let swidh = self.width as i32;
		let sheit = self.height as i32;
		let owidh = other.width as i32;
		let oheit = other.height as i32;
		if x > swidh || y > sheit || (x + owidh) < 0 || (y + oheit) < 0 {
			return Err(ImageError::OutOfBounds{x, y})
		}
		if self.format != other.format {
			return Err(ImageError::DifferentFormat);
		}
		let blit_view = BlitView::from((self as &Image, other, x, y));
		Ok(())
	}
	pub fn convert_to(&mut self, format: ImageFormat) -> Result<(), ImageError> {
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

	#[test]
	fn blitview_works() {
//
	}
}

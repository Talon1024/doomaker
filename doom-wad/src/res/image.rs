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
	pub fn alpha_equivalent(&self) -> ImageFormat {
		match self {
			ImageFormat::RGB => ImageFormat::RGBA,
			ImageFormat::RGBA => ImageFormat::RGBA,
			ImageFormat::Indexed => ImageFormat::IndexedAlpha,
			ImageFormat::IndexedAlpha => ImageFormat::IndexedAlpha
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
	#[error("This image, which is in {my} format, cannot be converted to {your} format.")]
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

#[derive(PartialEq, Eq, Debug)]
struct BlitView {
	/// Width of image A
	aw: usize,
	/// X coordinate of top left corner of view in image A
	ax: usize,
	/// Y coordinate of top left corner of view in image A
	ay: usize,
	/// Width of image B
	bw: usize,
	/// X coordinate of top left corner of view in image B
	bx: usize,
	/// Y coordinate of top left corner of view in image B
	by: usize,
	/// Image colour channel count
	channels: usize,
	/// Number of rows
	rows: usize,
	/// Number of columns
	cols: usize,
	/// Current row
	row: usize,
}

impl Iterator for BlitView {
	type Item = (Range<usize>, Range<usize>);
	fn next(&mut self) -> Option<Self::Item> {
		if self.row < self.rows {
			// Row * width + column
			let asta = ((self.ay + self.row) * self.aw + self.ax) * self.channels;
			let aend = asta + self.cols * self.channels;
			let bsta = ((self.by + self.row) * self.bw + self.bx) * self.channels;
			let bend = bsta + self.cols * self.channels;
			self.row += 1;
			Some((asta..aend, bsta..bend))
		} else {
			None
		}
	}
}

impl From<(&Image, &Image, i32, i32)> for BlitView {
	fn from(v: (&Image, &Image, i32, i32)) -> BlitView {
		BlitView {
			aw: v.0.width,
			ax: (v.2.max(0) as usize).min(v.0.width - 1),
			ay: (v.3.max(0) as usize).min(v.0.height - 1),
			bw: v.1.width,
			bx: ((-v.2).max(0)) as usize,
			by: ((-v.3).max(0)) as usize,
			channels: v.0.format.channels(),
			rows: {
				if v.3 < 0 {
					(v.1.height - v.3.abs() as usize).min(v.0.height)
				} else {
					(v.0.height - v.3 as usize).min(v.1.height)
				}
			},
			cols: {
				if v.2 < 0 {
					(v.1.width - v.2.abs() as usize).min(v.0.width)
				} else {
					(v.0.width - v.2 as usize).min(v.1.width)
				}
			},
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
		let blend: blending::BlendFunction = blending::mix;
		if self.format.alpha().is_none() {
			blit_view.for_each(|(ra, rb)| {
				let rowa = &mut self.data[ra];
				let rowb = &other.data[rb];
				rowa.copy_from_slice(rowb);
			});
		} else if let Some(alpha_index) = self.format.alpha() {
			blit_view.for_each(|(ra, rb)| {
				let channels = self.format.channels();
				let rowa = &mut self.data[ra];
				let rowb = &other.data[rb];
				rowa.chunks_exact_mut(channels)
					.zip(rowb.chunks_exact(channels))
					.for_each(|(pxa, pxb)| {
						let alpha = pxb.get(alpha_index).copied().unwrap();
						if alpha < 255 {
							let pxl = blend(pxa, pxb, Some(alpha_index));
							pxa.copy_from_slice(&pxl);
						} else {
							pxa.copy_from_slice(pxb);
						}
					});
			});
		}
		Ok(())
	}
	pub fn add_alpha(&mut self) -> Result<(), ImageError> {
		let alpha_format = self.format.alpha_equivalent();
		if self.format == alpha_format {
			return Ok(());
		}
		let default_alpha = 255;
		// TODO: Use intersperse when it's stable
		let new_data = |channels| {
			let mut data: Vec<u8> = Vec::with_capacity(
				self.width * self.height * channels);
			let mut opos: usize = 0; // Orig data position
			let mut npos: usize = 0; // New data position
			loop {
				let new_value = if ((npos+1) % channels) != 0 {
					let orig_value = self.data.get(opos);
					if orig_value.is_none() {
						break;
					}
					opos += 1;
					orig_value.copied().unwrap()
				} else {
					default_alpha
				};
				data.push(new_value);
				npos += 1;
			}
			data
		};
		let channels = alpha_format.channels();
		self.data = new_data(channels);
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
		let pal = pal.unwrap_or_else(Image::grayscale_palette);
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
	fn blitview_inside() {
		// Image A: 12 x 12 x 1 channel
		// Image B: 4  x 4  x 1 channel @ (4,4)
		let mut blit_view = BlitView {
			aw: 12,
			ax: 4,
			ay: 4,
			bw: 4,
			bx: 0,
			by: 0,
			channels: 1,
			rows: 4,
			cols: 4,
			row: 0,
		};
		let ima = Image {
			width: 12,
			height: 12,
			data: vec![0u8; 144],
			x: 0,
			y: 0,
			format: ImageFormat::Indexed
		};
		let imb = Image {
			width: 4,
			height: 4,
			data: vec![0u8; 16],
			x: 0,
			y: 0,
			format: ImageFormat::Indexed
		};
		assert_eq!(blit_view, BlitView::from((&ima, &imb, 4, 4)));
		assert_eq!(blit_view.next(), Some((52..56, 0..4)));
		assert_eq!(blit_view.next(), Some((64..68, 4..8)));
		assert_eq!(blit_view.next(), Some((76..80, 8..12)));
		assert_eq!(blit_view.next(), Some((88..92, 12..16)));
		assert_eq!(blit_view.next(), None);
	}

	#[test]
	fn blitview_neg_xy() {
		// Image A: 12 x 12 x 1 channel
		// Image B: 8  x 8  x 1 channel @ (-4,-4)
		let mut blit_view = BlitView {
			aw: 12,
			ax: 0,
			ay: 0,
			bw: 8,
			bx: 4,
			by: 4,
			channels: 1,
			rows: 4,
			cols: 4,
			row: 0,
		};
		let ima = Image {
			width: 12,
			height: 12,
			data: vec![0u8; 144],
			x: 0,
			y: 0,
			format: ImageFormat::Indexed
		};
		let imb = Image {
			width: 8,
			height: 8,
			data: vec![0u8; 64],
			x: 0,
			y: 0,
			format: ImageFormat::Indexed
		};
		assert_eq!(blit_view, BlitView::from((&ima, &imb, -4, -4)));
		assert_eq!(blit_view.next(), Some((0..4, 36..40)));
		assert_eq!(blit_view.next(), Some((12..16, 44..48)));
		assert_eq!(blit_view.next(), Some((24..28, 52..56)));
		assert_eq!(blit_view.next(), Some((36..40, 60..64)));
		assert_eq!(blit_view.next(), None);
	}

	#[test]
	fn blitview_oob_xy() {
		// Image A: 12 x 12 x 1 channel
		// Image B: 8  x 8  x 1 channel @ (8,8)
		let mut blit_view = BlitView {
			aw: 12,
			ax: 8,
			ay: 8,
			bw: 8,
			bx: 0,
			by: 0,
			channels: 1,
			rows: 4,
			cols: 4,
			row: 0,
		};
		let ima = Image {
			width: 12,
			height: 12,
			data: vec![0u8; 144],
			x: 0,
			y: 0,
			format: ImageFormat::Indexed
		};
		let imb = Image {
			width: 8,
			height: 8,
			data: vec![0u8; 64],
			x: 0,
			y: 0,
			format: ImageFormat::Indexed
		};
		assert_eq!(blit_view, BlitView::from((&ima, &imb, 8, 8)));
		assert_eq!(blit_view.next(), Some((104..108, 0..4)));
		assert_eq!(blit_view.next(), Some((116..120, 8..12)));
		assert_eq!(blit_view.next(), Some((128..132, 16..20)));
		assert_eq!(blit_view.next(), Some((140..144, 24..28)));
		assert_eq!(blit_view.next(), None);
	}

	#[test]
	fn blit() -> Result<(), ImageError> {
		// Image A: 12 x 12 x 1 channel
		// Image B: 4  x 4  x 1 channel @ (4,4)
		let mut ima = Image {
			width: 12,
			height: 12,
			data: Vec::from(include_bytes!("../../tests/data/BLITBACK.raw").as_slice()),
			x: 0,
			y: 0,
			format: ImageFormat::IndexedAlpha
		};
		let imb = Image {
			width: 4,
			height: 4,
			data: Vec::from(include_bytes!("../../tests/data/BLITFORE.raw").as_slice()),
			x: 0,
			y: 0,
			format: ImageFormat::IndexedAlpha
		};
		let imexpected = Image {
			width: 12,
			height: 12,
			data: Vec::from(include_bytes!("../../tests/data/BLITRESU.raw").as_slice()),
			x: 0,
			y: 0,
			format: ImageFormat::IndexedAlpha
		};
		ima.blit(&imb, 4, 4)?;
		assert_eq!(ima.data, imexpected.data);
		Ok(())
	}

	#[test]
	fn blit_transparent() -> Result<(), ImageError> {
		// Image A: 12 x 12 x 1 channel
		// Image B: 4  x 4  x 1 channel @ (4,4)
		let mut ima = Image {
			width: 12,
			height: 12,
			data: Vec::from(include_bytes!("../../tests/data/BLITBACK.raw").as_slice()),
			x: 0,
			y: 0,
			format: ImageFormat::IndexedAlpha
		};
		let imb = Image {
			width: 4,
			height: 4,
			data: Vec::from(include_bytes!("../../tests/data/BLITFOR2.raw").as_slice()),
			x: 0,
			y: 0,
			format: ImageFormat::IndexedAlpha
		};
		let imexpected = Image {
			width: 12,
			height: 12,
			data: Vec::from(include_bytes!("../../tests/data/BLITRSU2.raw").as_slice()),
			x: 0,
			y: 0,
			format: ImageFormat::IndexedAlpha
		};
		ima.blit(&imb, 4, 4)?;
		assert_eq!(ima.data, imexpected.data);
		Ok(())
	}

	#[test]
	fn convert_indexed_to_indexedalpha() -> Result<(), ImageError> {
		let mut orig_image = Image {
			width: 2,
			height: 2,
			x: 0,
			y: 0,
			format: ImageFormat::Indexed,
			data: vec![7, 5, 9, 3],
		};
		orig_image.add_alpha()?;
		assert_eq!(orig_image.data, vec![7, 255, 5, 255, 9, 255, 3, 255]);
		Ok(())
	}

	#[test]
	fn convert_rgb_to_rgba() -> Result<(), ImageError> {
		let mut orig_image = Image {
			width: 2,
			height: 2,
			x: 0,
			y: 0,
			format: ImageFormat::RGB,
			data: vec![
				255, 0, 0,		0, 255, 0,
				0, 0, 255,		128, 128, 128],
		};
		orig_image.add_alpha()?;
		assert_eq!(orig_image.data, vec![
			255, 0, 0, 255,		0, 255, 0, 255,
			0, 0, 255, 255,		128, 128, 128, 255]);
		Ok(())
	}
}

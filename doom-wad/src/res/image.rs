
#[derive(Debug, PartialEq)]
pub enum ImageFormat {
	RGB,
	RGBA,
	Indexed,
	IndexedAlpha
}

impl ImageFormat {
	pub fn channels(&self) -> usize {
		match self {
			ImageFormat::RGB => 3,
			ImageFormat::RGBA => 4,
			ImageFormat::Indexed => 1,
			ImageFormat::IndexedAlpha => 2,
		}
	}
}

pub struct Image {
	pub width: usize,
	pub height: usize,
	pub data: Vec<u8>,
	pub x: i32,
	pub y: i32,
	pub format: ImageFormat
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

pub fn xy_to_bufpos(x: usize, y: usize, w: usize, h: usize, channels: usize) -> Option<usize> {
	if x >= w {
		// No need to check y >= h because if it is, the calculated buffer
		// position will be greater than the calculated image size
		return None;
	}
	let size = w * h * channels;
	let pos = y * w * channels + x * channels;
	if pos < size {
		Some(pos)
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

		// 2 channels
		assert_eq!(xy_to_bufpos(4, 7, 128, 128, 2), Some(1800));
		assert_eq!(xy_to_bufpos(128, 7, 128, 128, 2), None);
		assert_eq!(xy_to_bufpos(4, 128, 128, 128, 2), None);
		assert_eq!(xy_to_bufpos(127, 127, 128, 128, 2), Some(32766));

		// 3 channels
		assert_eq!(xy_to_bufpos(4, 7, 128, 128, 3), Some(2700));
		assert_eq!(xy_to_bufpos(128, 7, 128, 128, 3), None);
		assert_eq!(xy_to_bufpos(4, 128, 128, 128, 3), None);
		assert_eq!(xy_to_bufpos(127, 127, 128, 128, 3), Some(49149));

		// 4 channels
		assert_eq!(xy_to_bufpos(4, 7, 128, 128, 4), Some(3600));
		assert_eq!(xy_to_bufpos(128, 7, 128, 128, 4), None);
		assert_eq!(xy_to_bufpos(4, 128, 128, 128, 4), None);
		assert_eq!(xy_to_bufpos(127, 127, 128, 128, 4), Some(65532));
	}
}

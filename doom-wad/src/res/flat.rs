// Flats are 64x64 graphics stored as indexed samples in row-major order.
use crate::wad::DoomWadLump;
use crate::res::{ToImage, Image, ImageDimension};
use std::sync::Arc;

use super::IndexedBuffer;

const MINIMUM_SIZE: usize = 64;
// const MINIMUM_BYTES: usize = 4096; // 64 * 64

pub struct FlatImage {
    lump: Arc<DoomWadLump>
}

impl FlatImage {
    pub fn height(&self) -> ImageDimension {
        let l = self.lump.data.len();
        let mut h = l / MINIMUM_SIZE;
        if l % MINIMUM_SIZE > 0 {
            h += 1;
        }
        h as ImageDimension
    }
    pub fn width(&self) -> ImageDimension {
        MINIMUM_SIZE.min(self.lump.data.len()) as ImageDimension
    }
}

impl ToImage for FlatImage {
    fn to_image(&self) -> Image {
        let height = self.height();
        let width = self.width();
        let len = self.lump.data.len();
        let buflen = (width * height) as usize;

        let mut data = vec![0; buflen];
        let _ = &data[..len].copy_from_slice(&self.lump.data);
        Image {
            width, height, indexed: Some(IndexedBuffer {
                buffer: data.into_boxed_slice(),
                alpha: false,
            }), truecolor: None, x: 0, y: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wad::LumpName;

    #[test]
    fn converts_flats_properly() {
        let flat_lump = Arc::new(DoomWadLump {
            name: LumpName::try_from("A").unwrap(),
            data: Vec::from(include_bytes!("../../tests/data/TLITE6_5.flt").as_slice())
        });
        let expected = Image {
            width: 64,
            height: 64,
            x: 0,
            y: 0,
            indexed: Some(IndexedBuffer {
                buffer: Box::from(include_bytes!(
                    "../../tests/data/TLITE6_5.flt").as_slice()),
                alpha: false,
            }),
            truecolor: None
        };

        let flat_image = FlatImage {lump: flat_lump};
        let flat_image = flat_image.to_image();

        assert_eq!(flat_image.width, expected.width);
        assert_eq!(flat_image.height, expected.height);
        assert_eq!(flat_image.indexed, expected.indexed);
    }

    #[test]
    // Heretic's F_SKY1 lump is only 4 bytes long
    fn converts_heretic_sky() {
        let flat_lump = Arc::new(DoomWadLump {
            name: LumpName::try_from("F_SKY1").unwrap(),
            data: Vec::<u8>::from([83, 75, 89, 10])
        });
        let expected = Image {
            width: 4,
            height: 1,
            x: 0,
            y: 0,
            indexed: Some(IndexedBuffer {
                buffer: Box::from([83, 75, 89, 10]),
                alpha: false,
            }),
            truecolor: None,
        };

        let flat_image = FlatImage {lump: flat_lump};
        let flat_image = flat_image.to_image();

        assert_eq!(flat_image.width, expected.width);
        assert_eq!(flat_image.height, expected.height);
        assert_eq!(flat_image.indexed, expected.indexed);
    }

    #[test]
    // How about a flat with only 1.5 rows?
    fn converts_incomplete_flat() {
        let flat_lump = Arc::new(DoomWadLump {
            name: LumpName::try_from("INCOMPLE").unwrap(),
            data: Vec::from(include_bytes!("../../tests/data/INCOMPLE.flt").as_slice())
        });
        let expected = Image {
            width: 64,
            height: 2,
            x: 0,
            y: 0,
            indexed: Some(IndexedBuffer {
                buffer: {
                    let mut v = Vec::from(include_bytes!("../../tests/data/INCOMPLE.flt").as_slice());
                    let vl = v.len();
                    v.extend(vec![0; 64 * 2 - vl]); // v should be 128 bytes long at this point
                    v.into_boxed_slice()
                },
                alpha: false,
            }),
            truecolor: None,
        };

        let flat_image = FlatImage {lump: flat_lump};
        let flat_image = flat_image.to_image();

        assert_eq!(flat_image.width, expected.width);
        assert_eq!(flat_image.height, expected.height);
        assert_eq!(flat_image.indexed, expected.indexed);
    }
}

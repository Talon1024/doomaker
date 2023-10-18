use crate::wad::DoomWadLump;
use crate::res::{self, ToImage, Image, ImageFormat, ImageDimension, IndexedBuffer};
use std::{
    error::Error,
    io::{Read, Cursor, Seek, SeekFrom},
    sync::Arc,
};
use binrw::BinRead;

#[derive(Debug, Clone)]
pub struct DoomPicture {
    lump: Arc<DoomWadLump>
}

impl From<Arc<DoomWadLump>> for DoomPicture {
    fn from(lump: Arc<DoomWadLump>) -> DoomPicture {
        DoomPicture { lump: Arc::clone(&lump) }
    }
}

#[derive(BinRead)]
#[br(little)]
struct PictureHeader
{
    width: u16,
    height: u16,
    x: i16,
    y: i16,
}

impl ToImage for DoomPicture {
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
        let bad_image = Image::default();

        let PictureHeader {width, height, x, y} = {
            let res = PictureHeader::read(&mut pos);
            if res.is_err() { return bad_image; }
            res.unwrap()
        };
        let width = width as ImageDimension;
        let height = height as ImageDimension;

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
                width, height, indexed: Some(IndexedBuffer {
                    buffer: pixels.into_boxed_slice(),
                    alpha: true,
                }), truecolor: None,
                x: x as i32, y: y as i32
            }
        } else {  // Fully opaque
            Image {
                width, height, indexed: Some(IndexedBuffer {
                    buffer: data.into_boxed_slice(),
                    alpha: false,
                }), truecolor: None,
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
        let patch_lump = Arc::new(DoomWadLump {
            name: LumpName::try_from("MOSSBRK8").unwrap(),
            data: Vec::from(include_bytes!("../../tests/data/MOSSBRK8.lmp").as_slice())
        });
        let expected = Image {
            width: 128,
            height: 128,
            x: 64,
            y: 123,
            indexed: Some(IndexedBuffer {
                buffer: Box::from(include_bytes!(
                    "../../tests/data/MOSSBRK8.raw").as_slice()),
                alpha: false
            }),
            truecolor: None,
        };

        let picture = DoomPicture {lump: Arc::clone(&patch_lump)};
        let image = picture.to_image();

        assert_eq!(image.width, expected.width);
        assert_eq!(image.height, expected.height);
        assert_eq!(image.x, expected.x);
        assert_eq!(image.y, expected.y);
        assert_eq!(image.indexed, expected.indexed);
        assert_eq!(image.truecolor, expected.truecolor);
    }

    #[test]
    fn converts_transparent_patches_correctly() {
        let patch_lump = Arc::new(DoomWadLump {
            name: LumpName::try_from("GRATE").unwrap(),
            data: Vec::from(include_bytes!("../../tests/data/GRATE.lmp").as_slice())
        });
        let expected = Image {
            width: 128,
            height: 128,
            x: 64,
            y: 123,
            indexed: Some(IndexedBuffer {
                buffer: Box::from(include_bytes!(
                    "../../tests/data/GRATE.raw").as_slice()),
                alpha: true,
            }),
            truecolor: None,
        };

        let picture = DoomPicture {lump: Arc::clone(&patch_lump)};
        let image = picture.to_image();

        assert_eq!(image.width, expected.width);
        assert_eq!(image.height, expected.height);
        assert_eq!(image.x, expected.x);
        assert_eq!(image.y, expected.y);
        assert_eq!(image.indexed, expected.indexed);
        assert_eq!(image.truecolor, expected.truecolor);
    }

    #[test]
    fn converts_tall_patches_correctly() {
        let patch_lump = Arc::new(DoomWadLump {
            name: LumpName::try_from("SHTGC0").unwrap(),
            data: Vec::from(include_bytes!("../../tests/data/SHTGC0.lmp").as_slice())
        });
        let expected = Image {
            width: 98,
            height: 146,
            x: -27,
            y: -22,
            indexed: Some(IndexedBuffer {
                buffer: Box::from(include_bytes!(
                    "../../tests/data/SHTGC0.raw").as_slice()),
                alpha: true,
            }),
            truecolor: None,
        };

        let picture = DoomPicture {lump: Arc::clone(&patch_lump)};
        let image = picture.to_image();

        assert_eq!(image.width, expected.width);
        assert_eq!(image.height, expected.height);
        assert_eq!(image.x, expected.x);
        assert_eq!(image.y, expected.y);
        assert_eq!(image.indexed, expected.indexed);
        assert_eq!(image.truecolor, expected.truecolor);
    }

    #[test]
    fn converts_deepsea_tall_patches_correctly() {
        let patch_lump = Arc::new(DoomWadLump {
            name: LumpName::try_from("CYBRE1").unwrap(),
            data: Vec::from(include_bytes!("../../tests/data/CYBRE1.lmp").as_slice())
        });
        let expected = Image {
            width: 277,
            height: 335,
            x: 138,
            y: 331,
            indexed: Some(IndexedBuffer {
                buffer: Box::from(include_bytes!(
                    "../../tests/data/CYBRE1.raw").as_slice()),
                alpha: true,
            }),
            truecolor: None,
        };

        let picture = DoomPicture {lump: Arc::clone(&patch_lump)};
        let image = picture.to_image();

        assert_eq!(image.width, expected.width);
        assert_eq!(image.height, expected.height);
        assert_eq!(image.x, expected.x);
        assert_eq!(image.y, expected.y);
        assert_eq!(image.indexed, expected.indexed);
        assert_eq!(image.truecolor, expected.truecolor);
    }

    #[test]
    fn converts_tswgb0_correctly() {
        let patch_lump = Arc::new(DoomWadLump {
            name: LumpName::try_from("TSWGB0").unwrap(),
            data: Vec::from(include_bytes!("../../tests/data/TSWGB0.lmp").as_slice())
        });
        let expected = Image {
            width: 179,
            height: 333,
            x: -249,
            y: 155,
            indexed: Some(IndexedBuffer {
                buffer: Box::from(include_bytes!(
                    "../../tests/data/TSWGB0.raw").as_slice()),
                alpha: true,
            }),
            truecolor: None
        };

        let picture = DoomPicture {lump: Arc::clone(&patch_lump)};
        let image = picture.to_image();

        assert_eq!(image.width, expected.width);
        assert_eq!(image.height, expected.height);
        assert_eq!(image.x, expected.x);
        assert_eq!(image.y, expected.y);
        assert_eq!(image.indexed, expected.indexed);
        assert_eq!(image.truecolor, expected.truecolor);
    }
}

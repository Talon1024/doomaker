use thiserror::Error;
use std::fmt::{Debug, Display};
use std::ops::Range;
use image::{RgbaImage, Pixel, Primitive};

mod blending;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ImageFormat {
    // RGB,
    RGBA,
    Indexed,
    IndexedAlpha
}

impl ImageFormat {
    pub fn channels(&self) -> usize {
        match self {
            // ImageFormat::RGB => 3,
            ImageFormat::RGBA => 4,
            ImageFormat::Indexed => 1,
            ImageFormat::IndexedAlpha => 2,
        }
    }
}

impl Display for ImageFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            // ImageFormat::RGB => write!(f, "RGB"),
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
    #[error("Both images need indexed data to \"blit\" one onto the other!")]
    DifferentFormat,
    /// The target image is outside of the original image's bounds
    #[error("({x} {y}) is outside of this image's boundaries!")]
    OutOfBounds { x: i32, y: i32 }
}

pub type ImageDimension = usize;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IndexedBuffer {
    pub buffer: Box<[u8]>,
    pub alpha: bool
}

#[derive(Debug, Clone, Default)]
pub struct Image {
    pub width: ImageDimension,
    pub height: ImageDimension,
    pub indexed: Option<IndexedBuffer>,
    pub truecolor: Option<RgbaImage>,
    pub x: i32,
    pub y: i32,
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
    /// Image A colour channel count
    achannels: usize,
    /// Image B colour channel count
    bchannels: usize,
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
            let asta = ((self.ay + self.row) * self.aw + self.ax) * self.achannels;
            let aend = asta + self.cols * self.achannels;
            let bsta = ((self.by + self.row) * self.bw + self.bx) * self.bchannels;
            let bend = bsta + self.cols * self.bchannels;
            self.row += 1;
            Some((asta..aend, bsta..bend))
        } else {
            None
        }
    }
}

impl BlitView {
    pub fn new(ima: &Image, imb: &Image, x: i32, y: i32) -> BlitView {
        BlitView {
            aw: ima.width,
            ax: (x.max(0) as usize).min(ima.width - 1),
            ay: (y.max(0) as usize).min(ima.height - 1),
            bw: imb.width,
            bx: ((-x).max(0)) as usize,
            by: ((-y).max(0)) as usize,
            achannels: match ima.indexed {
                Some(ref i) => if i.alpha {2} else {1},
                None => 0,
            },
            bchannels: match imb.indexed {
                Some(ref i) => if i.alpha {2} else {1},
                None => 0,
            },
            rows: {
                if y < 0 {
                    (imb.height - y.abs() as usize).min(ima.height)
                } else {
                    (ima.height - y as usize).min(imb.height)
                }
            },
            cols: {
                if x < 0 {
                    (imb.width - x.abs() as usize).min(ima.width)
                } else {
                    (ima.width - x as usize).min(imb.width)
                }
            },
            row: 0,
        }
    }
}

impl Image {
    pub fn new(width: ImageDimension, height: ImageDimension, format: ImageFormat) -> Image {
        let indexed = match format {
            ImageFormat::RGBA => None,
            ImageFormat::Indexed => Some(IndexedBuffer {
                buffer: Box::from(vec![0; width * height]),
                alpha: false,
            }),
            ImageFormat::IndexedAlpha => Some(IndexedBuffer {
                buffer: Box::from(vec![0; width * height * 2]),
                alpha: true,
            }),
        };
        let truecolor = match format {
            ImageFormat::RGBA => Some(
                RgbaImage::new(width as u32, height as u32)),
            ImageFormat::Indexed => None,
            ImageFormat::IndexedAlpha => None,
        };
        Image {
            width, height, x: 0, y: 0, indexed, truecolor,
        }
    }
    /// Draws another image on top of this one at the specified location
    /// 
    /// This method modifies the image in-place.
    pub fn blit(&mut self, other: &Image, x: i32, y: i32) -> Result<(), ImageError> {
        let swidh = self.width as i32;
        let sheit = self.height as i32;
        let owidh = other.width as i32;
        let oheit = other.height as i32;
        if x > swidh || y > sheit || (x + owidh) < 0 || (y + oheit) < 0 {
            return Err(ImageError::OutOfBounds{x, y})
        }
        let blit_view = BlitView::new(self, other, x, y);
        match (&mut self.indexed, &other.indexed) {
            (Some(ref mut me), Some(ref you)) => {
                let alphas = (me.alpha, you.alpha);
                match alphas {
                    (false, false) => {
                        blit_view.for_each(|(ra, rb)| {
                            let rowa = &mut me.buffer[ra];
                            let rowb = &you.buffer[rb];
                            rowa.copy_from_slice(rowb);
                        });
                    },
                    (true, true) => {
                        blit_view.for_each(|(ra, rb)| {
                            let rowa = &mut me.buffer[ra];
                            let rowb = &you.buffer[rb];
                            rowa.chunks_exact_mut(2).zip(
                            rowb.chunks_exact(2)).for_each(
                            |(pxa, pxb)| {
                                if pxb[1] != 0 {
                                    pxa.copy_from_slice(pxb);
                                }
                            });
                        });
                    },
                    (true, false) => {
                        blit_view.for_each(|(ra, rb)| {
                            let rowa = &mut me.buffer[ra];
                            let rowb = &you.buffer[rb];
                            rowa.chunks_exact_mut(2).zip(
                            rowb.chunks_exact(1)).for_each(
                            |(pxa, pxb)| {
                                pxa[0] = pxb[0];
                                pxa[1] = 255;
                            });
                        });
                    },
                    (false, true) => {
                        blit_view.for_each(|(ra, rb)| {
                            let rowa = &mut me.buffer[ra];
                            let rowb = &you.buffer[rb];
                            rowa.chunks_exact_mut(1).zip(
                            rowb.chunks_exact(2)).for_each(
                            |(pxa, pxb)| {
                                if pxb[1] != 0 {
                                    pxa[0] = pxb[0];
                                }
                            });
                        });
                    },
                }
                Ok(())
            },
            _ => Err(ImageError::DifferentFormat)
        }
    }
    /// Add an alpha channel to the indexed image data. Does not affect
    /// RGB/RGBA image data.
    /// 
    /// Returns whether or not an alpha channel was added.
    /// 
    /// This method modifies the image in-place.
    pub fn add_alpha(&mut self) -> bool {
        let default_alpha = 255;
        if self.indexed.is_none() {
            // self.truecolour has RGBA data
            return false;
        }
        let indexed = self.indexed.as_ref().unwrap();
        if indexed.alpha {
            // No need to add an alpha channel
            return false;
        }
        // TODO: Use intersperse when it's stable
        let new_data = {
            let mut data: Vec<u8> = vec![0; self.width * self.height * 2];
            data.chunks_exact_mut(2).zip(
            indexed.buffer.chunks_exact(1)).for_each(|(pxa, pxb)| {
                pxa[0] = pxb[0];
                pxa[1] = default_alpha;
            });
            data
        }.into_boxed_slice();
        self.indexed.replace(IndexedBuffer {
            buffer: new_data,
            alpha: true
        });
        true
    }
    /// Convert image to RGB or RGBA format using the given palette
    /// 
    /// If `pal` is None, a grayscale palette is used.
    /// 
    /// Returns whether or not the image was converted to RGB/RGBA format.
    /// Overwrites any existing RGB data.
    /// 
    /// This method modifies the image in-place.
    pub fn to_rgb(&mut self, pal: Option<[u8; 768]>) -> bool {
        let pal = pal.unwrap_or(super::GRAYSCALE_PALETTE);
        let get_colour = |index: u8| {
            let start = index as usize * 3;
            let end = start + 3;
            &pal[start..end]
        };
        if self.indexed.is_none() {
            // Cannot create RGB image without indexed pixel data
            return false;
        }
        let image = self.truecolor.insert(
            RgbaImage::new(self.width as u32, self.height as u32));
        let indexed = self.indexed.as_ref().unwrap();
        let palpha = if indexed.alpha {2} else {1};
        let pal_pixels = indexed.buffer.chunks_exact(palpha);
        image.pixels_mut().zip(pal_pixels).for_each(|(tp, ip)| {
            let rgb = get_colour(ip[0]);
            let mut component = 0;
            tp.apply_with_alpha(|_| {
                let channel = rgb[component];
                component += 1;
                channel
            }, |_| {
                *ip.get(1).unwrap_or(&u8::DEFAULT_MAX_VALUE)
            });
        });
        true
    }
}

pub trait ToImage {
    fn to_image(&self) -> Image;
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
            achannels: 1,
            bchannels: 1,
            rows: 4,
            cols: 4,
            row: 0,
        };
        let ima = Image {
            width: 12,
            height: 12,
            indexed: Some(IndexedBuffer {
                buffer: vec![0u8; 12 * 12].into_boxed_slice(),
                alpha: false,
            }),
            truecolor: None,
            x: 0,
            y: 0,
        };
        let imb = Image {
            width: 4,
            height: 4,
            indexed: Some(IndexedBuffer {
                buffer: vec![0u8; 16].into_boxed_slice(),
                alpha: false,
            }),
            truecolor: None,
            x: 0,
            y: 0,
        };
        assert_eq!(blit_view, BlitView::new(&ima, &imb, 4, 4));
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
            achannels: 1,
            bchannels: 1,
            rows: 4,
            cols: 4,
            row: 0,
        };
        let ima = Image {
            width: 12,
            height: 12,
            indexed: Some(IndexedBuffer {
                buffer: vec![0u8; 144].into_boxed_slice(),
                alpha: false,
            }),
            truecolor: None,
            x: 0,
            y: 0,
        };
        let imb = Image {
            width: 8,
            height: 8,
            indexed: Some(IndexedBuffer{
                buffer: vec![0u8; 64].into_boxed_slice(),
                alpha: false,
            }),
            truecolor: None,
            x: 0,
            y: 0,
        };
        assert_eq!(blit_view, BlitView::new(&ima, &imb, -4, -4));
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
            achannels: 1,
            bchannels: 1,
            rows: 4,
            cols: 4,
            row: 0,
        };
        let ima = Image {
            width: 12,
            height: 12,
            indexed: Some(IndexedBuffer{
                buffer: vec![0u8; 144].into_boxed_slice(),
                alpha: false,
            }),
            truecolor: None,
            x: 0,
            y: 0,
        };
        let imb = Image {
            width: 8,
            height: 8,
            indexed: Some(IndexedBuffer{
                buffer: vec![0u8; 64].into_boxed_slice(),
                alpha: false,
            }),
            truecolor: None,
            x: 0,
            y: 0,
        };
        assert_eq!(blit_view, BlitView::new(&ima, &imb, 8, 8));
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
            indexed: Some(IndexedBuffer {
                buffer: Box::from(include_bytes!(
                    "../../tests/data/BLITBACK.raw").as_slice()),
                alpha: true,
            }),
            truecolor: None,
            x: 0,
            y: 0,
        };
        let imb = Image {
            width: 4,
            height: 4,
            indexed: Some(IndexedBuffer {
                buffer: Box::from(include_bytes!(
                    "../../tests/data/BLITFORE.raw").as_slice()),
                alpha: true,
            }),
            truecolor: None,
            x: 0,
            y: 0,
        };
        let imexpected = Image {
            width: 12,
            height: 12,
            indexed: Some(IndexedBuffer {
                buffer: Box::from(include_bytes!(
                    "../../tests/data/BLITRESU.raw").as_slice()),
                alpha: true,
            }),
            truecolor: None,
            x: 0,
            y: 0,
        };
        ima.blit(&imb, 4, 4)?;
        assert_eq!(
            (&ima.indexed.unwrap()).buffer,
            (&imexpected.indexed.unwrap()).buffer
        );
        Ok(())
    }

    #[test]
    fn blit_transparent() -> Result<(), ImageError> {
        // Image A: 12 x 12 x 1 channel
        // Image B: 4  x 4  x 1 channel @ (4,4)
        let mut ima = Image {
            width: 12,
            height: 12,
            indexed: Some(IndexedBuffer {
                buffer: Box::from(include_bytes!(
                    "../../tests/data/BLITBACK.raw").as_slice()),
                alpha: true,
            }),
            truecolor: None,
            x: 0,
            y: 0,
        };
        let imb = Image {
            width: 4,
            height: 4,
            indexed: Some(IndexedBuffer {
                buffer: Box::from(include_bytes!(
                    "../../tests/data/BLITFOR2.raw").as_slice()),
                alpha: true,
            }),
            truecolor: None,
            x: 0,
            y: 0,
        };
        let imexpected = Image {
            width: 12,
            height: 12,
            indexed: Some(IndexedBuffer {
                buffer: Box::from(include_bytes!(
                    "../../tests/data/BLITRSU2.raw").as_slice()),
                alpha: true,
            }),
            truecolor: None,
            x: 0,
            y: 0,
        };
        ima.blit(&imb, 4, 4)?;
        assert_eq!(
            (&ima.indexed.unwrap()).buffer,
            (&imexpected.indexed.unwrap()).buffer
        );
        Ok(())
    }

    #[test]
    fn add_alpha_indexed() -> Result<(), ImageError> {
        let mut orig_image = Image {
            width: 2,
            height: 2,
            x: 0,
            y: 0,
            indexed: Some(IndexedBuffer {
                buffer: vec![7, 5, 9, 3].into_boxed_slice(),
                alpha:false
            }),
            truecolor: None,
        };
        orig_image.add_alpha();
        assert_eq!(
            (&orig_image.indexed.unwrap()).buffer,
            vec![7, 255, 5, 255, 9, 255, 3, 255].into_boxed_slice());
        Ok(())
    }

    #[test]
    fn add_alpha_rgb() -> Result<(), ImageError> {
        let mut orig_image = Image {
            width: 2,
            height: 2,
            x: 0,
            y: 0,
            indexed: None,
            truecolor: RgbaImage::from_vec(2, 2, vec![
                255, 0, 0, 255,		0, 255, 0, 255,
                0, 0, 255, 255,		128, 128, 128, 255]),
        };
        orig_image.add_alpha();
        assert_eq!(orig_image.truecolor, RgbaImage::from_vec(2, 2, vec![
            255, 0, 0, 255,		0, 255, 0, 255,
            0, 0, 255, 255,		128, 128, 128, 255]));
        Ok(())
    }

    #[test]
    fn convert_indexed_to_rgb() -> Result<(), ImageError> {
        let mut orig_image = Image {
            width: 2,
            height: 2,
            x: 0,
            y: 0,
            indexed: Some(IndexedBuffer {
                buffer: vec![144, 120, 237, 96].into_boxed_slice(),
                alpha: false,
            }),
            truecolor: None,
        };
        orig_image.to_rgb(Some(*include_bytes!("../../tests/data/PLAYPAL1.pal")));
        assert_eq!(
            orig_image.truecolor,
            RgbaImage::from_vec(2, 2, vec![
                255, 0, 0, 255,
                0, 255, 0, 255,
                0, 0, 255, 255,
                255, 255, 0, 255]));
        Ok(())
    }

    #[test]
    fn convert_indexedalpha_to_rgb() -> Result<(), ImageError> {
        let mut orig_image = Image {
            width: 2,
            height: 2,
            x: 0,
            y: 0,
            indexed: Some(IndexedBuffer {
                buffer: vec![144, 255, 120, 255, 237, 255, 96, 255].into_boxed_slice(),
                alpha: true,
            }),
            truecolor: None,
        };
        orig_image.to_rgb(Some(*include_bytes!("../../tests/data/PLAYPAL1.pal")));
        assert_eq!(orig_image.truecolor, RgbaImage::from_vec(2, 2, vec![
            255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 0, 255])
        );
        Ok(())
    }
}

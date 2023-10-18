// Palette
// All graphics in a Doom WAD are stored as palette indices rather than RGB or
// RGBA samples

use crate::wad::DoomWadLump;
use std::error::Error;
use std::fmt;
use std::sync::Arc;
use crate::res::{Image, ToImage, ImageDimension};
use image::RgbaImage;

const BYTES_PER_PALETTE: usize = 768; // 256 colors * RGB channels

type Palette = [u8; BYTES_PER_PALETTE];
// book/ch10-03-lifetime-syntax.html#lifetime-annotations-in-struct-definitions
pub struct PaletteCollection {
    lump: Arc<DoomWadLump>,
}

// rust-by-example/scope/lifetime/trait.html
impl From<Arc<DoomWadLump>> for PaletteCollection {
    fn from(lump: Arc<DoomWadLump>) -> PaletteCollection {
        PaletteCollection { lump: Arc::clone(&lump) }
    }
}

#[derive(Debug)]
enum PaletteError {
    NoPalettes
}
impl fmt::Display for PaletteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PaletteError::NoPalettes => {
                write!(f, "Palette collection has no palettes")?;
            }
        }
        Ok(())
    }
}
impl Error for PaletteError {}

impl PaletteCollection {
    pub fn count(&self) -> usize {
        self.lump.data.len() / BYTES_PER_PALETTE
    }
    pub fn get(&self, index: usize) -> Result<Palette, Box<dyn Error>> {
        let count = self.count();
        if count == 0 {
            return Err(Box::new(PaletteError::NoPalettes));
        }
        let start: usize = index * BYTES_PER_PALETTE;
        let end: usize = start + BYTES_PER_PALETTE;
        Palette::try_from(&self.lump.data[start..end]).map_err(Box::from)
    }
}

impl ToImage for PaletteCollection {
    fn to_image(&self) -> Image {
        let rows = self.count() as ImageDimension;
        let rgba: Vec<u8> = self.lump.data.chunks_exact(3)
        .flat_map(|ch| { [ch[0], ch[1], ch[2], 255] })
        .collect();
        Image {
            width: 256,
            height: rows,
            indexed: None,
            truecolor: RgbaImage::from_vec(256, rows as u32, rgba),
            x: 0, y: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wad::LumpName;

    #[test]
    fn imports_properly() {
        let playpal = Arc::new(DoomWadLump {
            name: LumpName::try_from("PLAYPAL").unwrap(),
            data: Vec::from(include_bytes!("../../tests/data/PLAYPAL.pal").as_slice()),
        });
        let palcol = PaletteCollection {lump: playpal};
        assert_eq!(palcol.count(), 14);
    }

    #[test]
    fn can_get_palettes() -> Result<(), Box<dyn Error>> {
        let playpal = Arc::new(DoomWadLump {
            name: LumpName::try_from("PLAYPAL").unwrap(),
            data: Vec::from(include_bytes!("../../tests/data/PLAYPAL.pal").as_slice()),
        });
        let palcol = PaletteCollection {lump: playpal};

        palcol.get(0)?;
        palcol.get(1)?;
        palcol.get(13)?;
        Ok(())
    }

    #[test]
    #[should_panic]
    fn bad_palette_index() -> () {
        let playpal = Arc::new(DoomWadLump {
            name: LumpName::try_from("PLAYPAL").unwrap(),
            data: Vec::from(include_bytes!("../../tests/data/PLAYPAL.pal").as_slice()),
        });
        let palcol = PaletteCollection {lump: playpal};

        // The palette collection has only 14 palettes, starting at 0. This
        // tries (and fails) to get palette #15.
        palcol.get(14).unwrap();
        ()
    }
}

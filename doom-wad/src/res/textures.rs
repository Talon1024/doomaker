// TEXTURE1, TEXTURE2, and PNAMES
use crate::wad::{self, DoomWadLump, LumpName, GetLump};
use crate::res::{Image, ImageFormat, ToImage};
use std::{
    collections::HashMap,
    error::Error,
    io::{Cursor, Read, Seek, SeekFrom},
    sync::Arc,
    num::NonZeroU8,
    ops::Deref
};
use ahash::RandomState;
use derive_deref::*;
use super::DoomPicture;
use bitflags::bitflags;

bitflags!{
    #[derive(Debug, Clone, Copy)]
    pub struct PatchFlags: i32 {}
}

#[derive(Debug, Clone)]
pub struct TexturePatch {
    patch: LumpName,
    x: i16, // X and Y offsets
    y: i16,
    flags: PatchFlags,
    lump: Option<DoomPicture>,
}

bitflags!{
    /// Texture flags for a TEXTUREx texture (ZDoom and derivatives only)
    /// See https://zdoom.org/wiki/TEXTUREx for more info
    #[derive(Debug, Clone, Copy)]
    pub struct TextureFlags: u16 {
        const WORLDPANNING = 0x8000;
    }
}

#[derive(Debug, Clone)]
pub struct Texture {
    name: LumpName,
    flags: TextureFlags,
    scalex: TextureScale,
    scaley: TextureScale,
    width: u16,
    height: u16,
    patches: Vec<TexturePatch>,
}

/// Texture scale for a TEXTUREx texture (ZDoom and derivatives only)
/// See https://zdoom.org/wiki/TEXTUREx
#[derive(Debug, Clone, Copy)]
pub struct TextureScale(NonZeroU8);

impl From<Option<NonZeroU8>> for TextureScale {
    fn from(value: Option<NonZeroU8>) -> Self {
        TextureScale(value.unwrap_or(NonZeroU8::new(8).unwrap()))
    }
}

impl TextureScale {
    pub fn to_dim_scale(&self) -> f32 {
        8. / (self.0.get() as f32)
    }
    pub fn to_uv_scale(&self) -> f32 {
        (self.0.get() as f32) / 8.
    }
    pub fn new(v: u8) -> Self {
        Self::from(NonZeroU8::new(v))
    }
}

impl Deref for TextureScale {
    type Target = NonZeroU8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct TextureDefinitions {
    textures: Vec<Arc<Texture>>,
}

impl TextureDefinitions {
    pub fn tex_map(&self) -> TextureDefinitionsMap {
        let mut map = TextureDefinitionsMap::default();
        self.textures.iter().for_each(|t| {
            map.insert(t.name, Arc::clone(t));
        });
        map
    }
}

#[derive(Default, Deref, Debug)]
pub struct TextureDefinitionsLumps(pub(crate) Vec<TextureDefinitions>);

impl TextureDefinitionsLumps {
    pub fn tex_map(&self) -> TextureDefinitionsMap {
        self.0.iter().map(TextureDefinitions::tex_map).reduce(|mut a, b| {
            a.extend(b); a
        }).unwrap_or_default()
    }
}

// https://users.rust-lang.org/t/hashmap-of-a-vector-of-objects/29220
// My solution is to add a method to the referred type which creates a HashMap
// of references to the data in the referree.
pub type TextureDefinitionsMap =
    HashMap<LumpName, Arc<Texture>, RandomState>;

// The reference is not held for long, since this function is private, and
// returns an owned value
fn read_pnames(pnames: &wad::DoomWadLump) ->
    Result<Vec<LumpName>, Box<dyn Error>>
{
    let mut num_buffer: [u8; 4] = [0; 4];
    let mut name_buffer: [u8; 8] = [0; 8];
    let mut pos = Cursor::new(&pnames.data);
    let name_count: usize = {
        pos.read_exact(&mut num_buffer)?;
        u32::from_le_bytes(num_buffer) as usize
    };
    (0..name_count).map(|_| {
        pos.read_exact(&mut name_buffer)?;
        LumpName::try_from(&name_buffer).map_err(Box::from)
    }).collect()
}

pub fn read_texturex<'a>(
    list: &Arc<DoomWadLump>, pnames: &Arc<DoomWadLump>, wad: &dyn GetLump) ->
    Result<TextureDefinitions, Box<dyn Error>>
{
    let patches = read_pnames(pnames)?;
    let mut pos = Cursor::new(&list.data);
    let mut name_buffer: [u8; 8] = [0; 8];
    let mut num_buffer: [u8; 4] = [0; 4];
    let mut short_buffer: [u8; 2] = [0; 2];
    let count: usize = {
        pos.read_exact(&mut num_buffer)?;
        u32::from_le_bytes(num_buffer) as usize
    };
    let tex_def_pos: Vec<u64> = (0..count)
    .map(|_| -> Result<u64, Box<dyn Error>> {
        pos.read_exact(&mut num_buffer)?;
        Ok(u32::from_le_bytes(num_buffer) as u64)
    }).collect::<Result<Vec<u64>, Box<dyn Error>>>()?;
    let mut defs = TextureDefinitions {
        textures: Vec::with_capacity(count),
    };
    tex_def_pos.into_iter().try_for_each(|offset| -> Result<(), Box<dyn Error>> {
        pos.seek(SeekFrom::Start(offset))?;
        // Name (8 bytes)
        pos.read_exact(&mut name_buffer)?; // name
        let name = LumpName::try_from(&name_buffer)?;
        // Flags (4 bytes)
        pos.read_exact(&mut short_buffer)?; // masked
        let flags = TextureFlags::from_bits_retain(
            u16::from_le_bytes(short_buffer));
        pos.read_exact(&mut num_buffer[0..1])?;
        let scalex = TextureScale::new(num_buffer[0]);
        pos.read_exact(&mut num_buffer[0..1])?;
        let scaley = TextureScale::new(num_buffer[0]);
        pos.read_exact(&mut short_buffer)?; // width
        let width = u16::from_le_bytes(short_buffer);
        pos.read_exact(&mut short_buffer)?; // height
        let height = u16::from_le_bytes(short_buffer);
        pos.seek(SeekFrom::Current(4))?; // skip columndirectory
        pos.read_exact(&mut short_buffer)?; // patchcount
        let patch_count = u16::from_le_bytes(short_buffer);
        defs.textures.push(Arc::new(Texture {
            name: name.clone(),
            flags,
            scalex,
            scaley,
            width,
            height,
            patches: (0..patch_count).map(|_| -> Result<TexturePatch, Box<dyn Error>> {
                pos.read_exact(&mut short_buffer)?;
                let x = i16::from_le_bytes(short_buffer);
                pos.read_exact(&mut short_buffer)?;
                let y = i16::from_le_bytes(short_buffer);
                pos.read_exact(&mut short_buffer)?;
                let pindex = u16::from_le_bytes(short_buffer);
                let patch_name = patches.get(pindex as usize).copied()
                    .ok_or(String::from("Invalid patch index!"))?;
                pos.read_exact(&mut num_buffer)?;
                // Two unused 16-bit integers
                let flags = PatchFlags::from_bits_retain(
                    i32::from_le_bytes(num_buffer));
                Ok(TexturePatch {
                    patch: patch_name,
                    x,
                    y, 
                    flags,
                    lump: wad.get_lump(patch_name).map(DoomPicture::from)
                })
            }).collect::<Result<Vec<TexturePatch>, Box<dyn Error>>>()?
        }));
        Ok(())
    })?;
    Ok(defs)
}

impl ToImage for Texture {
    fn to_image(&self) -> Image {
        let mut image = Image::new(self.width as usize, self.height as usize, ImageFormat::IndexedAlpha);
        self.patches.iter().for_each(|pa| {
            match &pa.lump {
                Some(lump) => {
                    let patch_image = lump.to_image();
                    let blit_res = image.blit(&patch_image, pa.x as i32, pa.y as i32);
                    if let Err(e) = blit_res {
                        eprintln!("{}", e);
                    }
                },
                None => (),
            };
        });
        image
    }
}


#[cfg(test)]
mod tests {

    use crate::wad::{DoomWadType, DoomWad};
    use super::*;

    #[test]
    fn reads_texturex() -> Result<(), Box<dyn Error>> {
        let texture1_name = LumpName(*b"TEXTURE1");
        let pnames_name = LumpName(*b"PNAMES\0\0");
        let wad = DoomWad {
            wtype: DoomWadType::PWAD,
            lumps: vec![Arc::new(DoomWadLump {
                name: texture1_name,
                data: Vec::from(*include_bytes!("../../tests/data/TEXTURE1.lmp"))
            }), Arc::new(DoomWadLump {
                name: pnames_name,
                data: Vec::from(*include_bytes!("../../tests/data/PNAMES.lmp"))
            })]
        };
        let texture_lump = wad.get_lump(texture1_name).ok_or("No TEXTURE1!")?;
        let pnames_lump = wad.get_lump(pnames_name).ok_or("No PNAMES!")?;
        let texdefs = read_texturex(&texture_lump, &pnames_lump, &wad)?;
        assert_eq!(texdefs.textures.len(), 4);
        assert_eq!(texdefs.textures[0].name, LumpName(*b"S3DUMMY\0"));
        Ok(())
    }
}

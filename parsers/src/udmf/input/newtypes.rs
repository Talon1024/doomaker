use std::str::FromStr;
use std::fmt::Display;

pub(crate) trait OptionalUDMFData : PartialEq + Eq + Default {
    fn is_default(&self) -> bool {
        self == &Self::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LightLevel(pub i32);

impl Default for LightLevel {
    fn default() -> Self {
        Self(160)
    }
}

impl FromStr for LightLevel {
    type Err = <i32 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(i32::from_str(s)?))
    }
}

impl OptionalUDMFData for LightLevel {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SidedefTexture(pub String);

impl Default for SidedefTexture {
    fn default() -> Self {
        Self(String::from("-"))
    }
}

impl FromStr for SidedefTexture {
    type Err = <String as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(String::from_str(s)?))
    }
}

macro_rules! impl_display_for_udmf_datum {
    // I copied this expression from the Rust standard library
    ($($t: ty)*) => {$(
        impl Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if !self.is_default() {
                    write!(f, "{}", self.0)
                } else {
                    Ok(())
                }
            }
        }
    )*};
}

impl OptionalUDMFData for SidedefTexture {}

impl_display_for_udmf_datum!{ SidedefIndex SidedefTexture LightLevel }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SidedefIndex(pub i32);

impl Default for SidedefIndex {
    fn default() -> Self {
        Self(-1)
    }
}

impl FromStr for SidedefIndex {
    type Err = <i32 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(i32::from_str(s)?))
    }
}

impl OptionalUDMFData for SidedefIndex {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MultiplicativeColour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Default for MultiplicativeColour {
    fn default() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
        }
    }
}

impl OptionalUDMFData for MultiplicativeColour {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// Default can just be derived here, since the numbers for additive colours
// are supposed to be 0 by default
pub struct AdditiveColour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl OptionalUDMFData for AdditiveColour {}

macro_rules! impls_for_colour {
    ($($t: ty)*) => {$(
        impl FromStr for $t {
            type Err = <u32 as FromStr>::Err;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let (start, radix) = match s {
                    _ if s.starts_with("0x") => (2, 16),
                    _ if s.starts_with("0") => (1, 8),
                    _ => (0, 10),
                };
                let s = &s[start..];
                let rgba = u32::from_str_radix(s, radix)?;
                let r = (rgba & 0x00FF0000u32) >> 16;
                let r = r as u8;
                let g = (rgba & 0x0000FF00u32) >> 8;
                let g = g as u8;
                let b = (rgba & 0x000000FFu32) >> 0;
                let b = b as u8;
                Ok(Self {
                    r,
                    g,
                    b,
                })
            }
        }
        impl Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if !self.is_default() {
                    let combined: u32 = ((self.r as u32) << 16)
                        | ((self.g as u32) << 8)
                        | self.b as u32;
                    write!(f, "{combined}")
                } else {
                    Ok(())
                }
            }
        }
    )*};
}

impls_for_colour! { MultiplicativeColour AdditiveColour }

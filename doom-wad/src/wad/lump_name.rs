use thiserror::Error;
use std::str;
use binrw::BinRead;

// TODO: use std::ascii::Char when it gets stabilized
// https://doc.rust-lang.org/std/ascii/enum.Char.html
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, BinRead)]
pub struct LumpName(pub(crate) [u8; 8]);

impl LumpName {
    fn zero_pos(&self) -> usize {
        self.0.iter().position(|&b| b == 0).unwrap_or(self.0.len())
    }
    pub fn as_str(&self) -> &str {
        // LumpName should be ASCII
        let zero_pos = self.zero_pos();
        str::from_utf8(&self.0[..zero_pos]).unwrap()
    }
}

#[derive(Error, Debug, Clone)]
pub enum LumpNameConvertError {
    #[error("Invalid ASCII character `{0}`")]
    InvalidASCII(u8),
    #[error("Characters after first NULL")]
    CharsAfterNull
}
/*
impl From<&[u8]> for LumpName {
    fn from(name: &[u8]) -> Self {
        let mut lump_name = Self::default();
        let slen = name.len().min(8);
        let (mut left, _) = lump_name.0.split_at_mut(slen);
        left.copy_from_slice(&name[..slen]);
        lump_name.0.map(|b| b & !32);
        lump_name
    }
}
*/
impl TryFrom<&[u8]> for LumpName {
    type Error = LumpNameConvertError;
    fn try_from(name: &[u8]) -> Result<Self, LumpNameConvertError> {
        let mut lump_name = Self::default();
        let slen = name.len().min(8);
        let (left, _) = lump_name.0.split_at_mut(slen);
        left.copy_from_slice(&name[..slen]);
        let mut nullhit = false;
        lump_name.0.iter().copied().fold(Ok(()), |b, c| {
            if c == 0 {
                nullhit = true;
                b.and(Ok(()))
            } else if nullhit && c != 0 {
                b.and(Err(LumpNameConvertError::CharsAfterNull))
            } else if c != 0 && c.is_ascii_control() {
                b.and(Err(LumpNameConvertError::InvalidASCII(c)))
            } else if c > 0x7F {
                b.and(Err(LumpNameConvertError::InvalidASCII(c)))
            } else {
                b.and(Ok(()))
            }
        })?;
        // Capitalize
        lump_name.0 = lump_name.0.map(|b| b.to_ascii_uppercase());
        Ok(lump_name)
    }
}

impl TryFrom<&str> for LumpName {
    type Error = LumpNameConvertError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(AsRef::<[u8]>::as_ref(value))
    }
}

impl TryFrom<&[u8; 8]> for LumpName {
    type Error = LumpNameConvertError;
    fn try_from(value: &[u8; 8]) -> Result<Self, Self::Error> {
        Self::try_from(AsRef::<[u8]>::as_ref(value))
    }
}

impl std::fmt::Display for LumpName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let first_zero = self.0.iter().position(|&v| v == 0)
            .unwrap_or(self.0.len());
        let string = std::str::from_utf8(&self.0[..first_zero]).map_err(|_| {
                std::fmt::Error})?;
        write!(f, "{}", string)
    }
}

impl std::fmt::Debug for LumpName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let first_zero = self.0.iter().position(|&v| v == 0)
            .unwrap_or(self.0.len());
        let string = std::str::from_utf8(&self.0[..first_zero]).map_err(|_| {
                std::fmt::Error})?;
        write!(f, "LumpName({})", string)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::error::Error;

    #[test]
    fn lump_name_to_string() -> Result<(), Box<dyn Error>> {
        let short_name = LumpName::try_from(b"CRAP\0\0\0\0".as_slice())?;
        let crap = short_name.to_string();
        assert_eq!(crap.len(), 4);
        let long_name = LumpName::try_from(b"NUTSCASE".as_slice())?;
        let nutcase = long_name.to_string();
        assert_eq!(nutcase.len(), 8);
        Ok(())
    }

    #[test]
    fn string_to_lump_name() -> Result<(), Box<dyn Error>> {
        use LumpNameConvertError::{self, *};
        let orig_name = LumpName::try_from(b"CRAP\0\0\0\0".as_slice())?;
        let lump_name = LumpName::try_from("Crap")?;
        assert_eq!(lump_name, orig_name);

        let lump_name = LumpName::try_from("METALT2")?;
        let orig_name = LumpName(*b"METALT2\0");
        assert_eq!(lump_name, orig_name);

        let lump_name = LumpName::try_from("Superduper")?;
        let orig_name = LumpName::try_from(b"SUPERDUP".as_slice())?;
        assert_eq!(lump_name, orig_name);

        let lump_name = LumpName::try_from("TRUCKING")?;
        let orig_name = LumpName::try_from(b"TRUCKING".as_slice())?;
        assert_eq!(lump_name, orig_name);

        let invalid_name = LumpName::try_from("😈's lair");
        assert!(matches!(invalid_name, Err(InvalidASCII(_))));

        Ok(())
    }
}

pub fn lump_name(slice: &[u8]) -> Vec<u8> {
    let mut vec = Vec::from(slice.clone());
    vec.resize({
        let mut iter = vec.iter();
        let zero_pos = iter.position(|v| v == &0u8);
        zero_pos.unwrap_or(vec.len())
    }, 0);
    vec
}

pub fn str_to_lump_name(name: &str) -> [u8; 8] {
    let mut lump_name: [u8; 8] = [0; 8];
    let slen = name.len().min(8);
    let (left, _) = lump_name.split_at_mut(slen);
    left.copy_from_slice(&name.as_bytes()[0..slen]);
    lump_name
}

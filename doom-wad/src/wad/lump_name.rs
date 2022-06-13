use std::error::Error;
use thiserror::Error;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct LumpName(pub [u8; 8]);

#[derive(Debug, Error)]
pub enum LumpNameConvertError {
	#[error("Invalid ASCII character")]
	InvalidASCII(u8),
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
		let (mut left, _) = lump_name.0.split_at_mut(slen);
		left.copy_from_slice(&name[..slen]);
		let mut nullhit = false;
		if let Some(c) = lump_name.0.iter().copied()
			.find(|&c| {
				if c == 0 {
					nullhit = true;
					false
				} else if nullhit && c != 0 {
					true
				} else if c != 0 && c.is_ascii_control() {
					true
				} else if c > 0x7F {
					true
				} else {
					false
				}}) {
			return Err(LumpNameConvertError::InvalidASCII(c));
		}
		lump_name.0 = lump_name.0.map(|b| b & !32); // Capitalize
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

impl From<LumpName> for &[u8] {
	fn from(v: LumpName) -> Self {
		v.0.as_slice()
	}
}

#[cfg(test)]
mod tests {

use super::*;

	#[test]
	fn lump_name_to_string() -> Result<(), Box<dyn Error>> {
		let short_name = LumpName::try_from([0x43, 0x52, 0x41, 0x50, 0, 0, 0, 0].as_slice())?; // CRAP
		let crap = short_name.to_string();
		assert_eq!(crap.len(), 4);
		let long_name = LumpName::try_from([0x4E, 0x55, 0x54, 0x53, 0x43, 0x41, 0x53, 0x45].as_slice())?; // NUTSCASE
		let nutcase = long_name.to_string();
		assert_eq!(nutcase.len(), 8);
		Ok(())
	}

	#[test]
	fn string_to_lump_name() -> Result<(), Box<dyn Error>> {
		let orig_name = LumpName::try_from([ // CRAP
			0x43, 0x52, 0x41, 0x50, 0, 0, 0, 0].as_slice())?;
		let lump_name = LumpName::try_from("Crap")?;
		assert_eq!(lump_name, orig_name);

		let lump_name = LumpName::try_from("Superduper")?;
		let orig_name = LumpName::try_from( // SUPERDUP
			b"SUPERDUP".as_slice())?;
		assert_eq!(lump_name, orig_name);

		let lump_name = LumpName::try_from("TRUCKING")?;
		let orig_name = LumpName::try_from( // TRUCKING
			b"TRUCKING".as_slice())?;
		assert_eq!(lump_name, orig_name);

		let invalid_name = LumpName::try_from("😈's lair");
		assert!(matches!(invalid_name, Err(_)));

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
	let (mut left, _) = lump_name.split_at_mut(slen);
	left.copy_from_slice(&name.as_bytes()[0..slen]);
	lump_name
}

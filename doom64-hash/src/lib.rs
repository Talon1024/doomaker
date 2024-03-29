#![warn(clippy::all)]
//! Doom 64 texture name hashing
use std::num::Wrapping;
// Doom 64 texture hash algorithm - C++
/*
hash_(1315423911)
for (int i = 0; i < 8 && str[i]; ++i) {
    int c = str[i];
    hash_ ^= (hash_ << 5) + toupper(c) + (hash_ >> 2);
}
hash_ &= 0xffff;
*/

/// Compute hash value for a texture name. This hash value is used by Doom 64
/// to look up the texture name for a certain side of a line.
/// 
/// # Example
/// 
/// ```
/// use doom64_hash::hash;
/// 
/// let name = String::from("SPACEB");
/// let hash = hash(&name);
/// assert_eq!(hash, 44097);
/// ```
pub fn hash(name: impl AsRef<[u8]>) -> u16 {
    let mut hash: Wrapping<u32> = Wrapping(1315423911);
    name.as_ref().iter().take(8).for_each(|c| {
        hash ^= (hash << 5) +
            Wrapping(c.to_ascii_uppercase() as u32) +
            (hash >> 2);
    });
    (hash.0 & 0xffff) as u16
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    /*
    use std::ffi::{CString, NulError};
    use std::os::raw::c_char;

    #[link(name = "doom64hash")]
    extern {
        #[link_name = "hash"]
        fn c_hash(name: *const c_char) -> u16;
    }

    #[test]
    fn hash_is_equal() -> Result<(), Box<dyn Error>> {

        let texnames: &[&str] = vec!["C102", "C200", "C307B", "C403", "H10", "H51", "HDOR10A", "HELLAJ", "HELLAS", "SDOORD", "SMONBC", "SPACEAI", "BITCHING", "F", "IMADETHISTESTFAILHAHAHAHA", "?", "-", "YOUFAILED"];

        texnames.iter().try_for_each(|&name| -> Result<(), NulError> {
            let my_hash = hash(name);
            let name = CString::new(name)?;
            let orig_hash = unsafe { c_hash(name.as_ptr()) };
            assert_eq!(my_hash, orig_hash);
            Ok(())
        })?;

        Ok(())
    }
    */

    #[test]
    fn hash_match() -> Result<(), Box<dyn Error>> {
        let texnames = ["H77", "?", "SPACEB"];
        let hashes: [u16; 3] = [20269, 111, 44097];

        texnames.iter().zip(hashes.iter()).try_for_each(
        |(&name, &ohash)| -> Result<(), String> {
            let myhash = hash(name);
            if myhash != ohash {
                Err(format!("Hash {} does not match {}", myhash, ohash))
            } else {
                Ok(())
            }
        })?;
        Ok(())
    }
}

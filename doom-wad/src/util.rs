use std::io::{Read, Result};

pub trait ReadFromReader : Sized {
    fn read(reader: &mut impl Read) -> Result<Self>;
}

#[cfg(test)]
mod tests {
    use std::{
        error::Error,
        fs::File,
        io::Read, str::FromStr,
    };
    use parsers::udmf::input::UDMFMap;
    #[test]
    fn can_read_udmf_map() -> Result<(), Box<dyn Error>> {
        let mut udmf_text = String::new();
        File::open("tests/basic_textmap.txt")?
            .read_to_string(&mut udmf_text)?;
        let udmf_text = udmf_text;
        let _udmf_map = UDMFMap::from_str(&udmf_text)?;
        assert_eq!(_udmf_map.namespace, "zdoom");
        Ok(())
    }
}

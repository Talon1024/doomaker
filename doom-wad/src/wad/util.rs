#[cfg(test)]
mod tests {
	#[test]
	fn lump_name_to_string() {
		use super::lump_name;
		let short_name: [u8; 8] = [0x43, 0x52, 0x41, 0x50, 0, 0, 0, 0]; // CRAP
		let crap = String::from_utf8(lump_name(&short_name)).unwrap();
		assert_eq!(crap.len(), 4);
		let long_name: [u8; 8] = [0x4E, 0x55, 0x54, 0x53, 0x43, 0x41, 0x53, 0x45]; // NUTSCASE
		let nutcase = String::from_utf8(lump_name(&long_name)).unwrap();
		assert_eq!(nutcase.len(), 8);
	}

	#[test]
	fn string_to_lump_name() {
		use super::to_lump_name;
		let orig_name: [u8; 8] = [ // CRAP
			0x43, 0x52, 0x41, 0x50, 0, 0, 0, 0];
		let name = String::from("Crap");
		let lump_name = to_lump_name(&name);
		assert_eq!(lump_name, orig_name);
		let name = String::from("Superduper");
		let orig_name: [u8; 8] = [ // SUPERDUP
			0x53, 0x55, 0x50, 0x45, 0x52, 0x44, 0x55, 0x50];
		let lump_name = to_lump_name(&name);
		assert_eq!(lump_name, orig_name);
		let name = String::from("b00BAfe3t");
		let orig_name: [u8; 8] = [ // B00BAFE3
			0x42, 0x30, 0x30, 0x42, 0x41, 0x46, 0x45, 0x33];
		let lump_name = to_lump_name(&name);
		assert_eq!(lump_name, orig_name);
		/*
		let invalid_name = String::from("😈's lair");
		let hornets = to_lump_name(&invalid_name);
		// assert!(if let Err = hornets);
		*/
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

pub fn to_lump_name(name: &String) -> [u8; 8] {
	let mut lump_name: [u8; 8] = [0; 8];
	let mut name_chars = name.bytes();
	for boot in 0..8 {
		// & (!32) is a cheap way of "capitalizing" a byte
		let byte = name_chars.next().unwrap_or(0);
		lump_name[boot] = if byte > 0x5F {byte & (!32)} else {byte};
	}
	lump_name
}

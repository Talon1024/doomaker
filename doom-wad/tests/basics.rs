#[cfg(test)]
mod tests {
	use doomwad::wad::*;
	use std::error::Error;
	use std::mem::discriminant;
	use std::fs::File;
	use std::io::Read;

	#[test]
	fn can_load_wad() -> Result<(), Box<dyn Error>> {
		let wad = DoomWad::load("tests/data/3difytest.wad")?;
		assert_eq!(discriminant(&wad.wtype), discriminant(&DoomWadType::PWAD));
		assert_ne!(wad.lumps.len(), 0);

		let lump_names: [&str; 272] = [
		"PLAYPAL", "COLORMAP", "P_START", "BAAAAAAD", "METALT2", "4DOT",
		"4DOTR", "4DOTG", "GOODGRIE", "LUNPOEG", "TRIMM", "P_END",
		"FF_START", "METALTF2", "METALT2", "F_END", "PNAMES", "TEXTURE1",
		"MAP01", "THINGS", "LINEDEFS", "SIDEDEFS", "VERTEXES", "SEGS",
		"SSECTORS", "NODES", "SECTORS", "REJECT", "BLOCKMAP", "MAP02",
		"THINGS", "LINEDEFS", "SIDEDEFS", "VERTEXES", "SEGS", "SSECTORS",
		"NODES", "SECTORS", "REJECT", "BLOCKMAP", "MAP03", "THINGS",
		"LINEDEFS", "SIDEDEFS", "VERTEXES", "SEGS", "SSECTORS", "NODES",
		"SECTORS", "REJECT", "BLOCKMAP", "MAP04", "THINGS", "LINEDEFS",
		"SIDEDEFS", "VERTEXES", "SEGS", "SSECTORS", "NODES", "SECTORS",
		"REJECT", "BLOCKMAP", "MAP05", "THINGS", "LINEDEFS", "SIDEDEFS",
		"VERTEXES", "SEGS", "SSECTORS", "NODES", "SECTORS", "REJECT",
		"BLOCKMAP", "MAP06", "THINGS", "LINEDEFS", "SIDEDEFS", "VERTEXES",
		"SEGS", "SSECTORS", "NODES", "SECTORS", "REJECT", "BLOCKMAP",
		"MAP07", "THINGS", "LINEDEFS", "SIDEDEFS", "VERTEXES", "SEGS",
		"SSECTORS", "NODES", "SECTORS", "REJECT", "BLOCKMAP", "MAP08",
		"THINGS", "LINEDEFS", "SIDEDEFS", "VERTEXES", "SEGS", "SSECTORS",
		"NODES", "SECTORS", "REJECT", "BLOCKMAP", "MAP09", "THINGS",
		"LINEDEFS", "SIDEDEFS", "VERTEXES", "SEGS", "SSECTORS", "NODES",
		"SECTORS", "REJECT", "BLOCKMAP", "MAP10", "THINGS", "LINEDEFS",
		"SIDEDEFS", "VERTEXES", "SEGS", "SSECTORS", "NODES", "SECTORS",
		"REJECT", "BLOCKMAP", "MAP11", "THINGS", "LINEDEFS", "SIDEDEFS",
		"VERTEXES", "SEGS", "SSECTORS", "NODES", "SECTORS", "REJECT",
		"BLOCKMAP", "MAP12", "THINGS", "LINEDEFS", "SIDEDEFS", "VERTEXES",
		"SEGS", "SSECTORS", "NODES", "SECTORS", "REJECT", "BLOCKMAP",
		"MAP13", "THINGS", "LINEDEFS", "SIDEDEFS", "VERTEXES", "SEGS",
		"SSECTORS", "NODES", "SECTORS", "REJECT", "BLOCKMAP", "MAP14",
		"THINGS", "LINEDEFS", "SIDEDEFS", "VERTEXES", "SEGS", "SSECTORS",
		"NODES", "SECTORS", "REJECT", "BLOCKMAP", "MAP15", "THINGS",
		"LINEDEFS", "SIDEDEFS", "VERTEXES", "SEGS", "SSECTORS", "NODES",
		"SECTORS", "REJECT", "BLOCKMAP", "MAP16", "THINGS", "LINEDEFS",
		"SIDEDEFS", "VERTEXES", "SEGS", "SSECTORS", "NODES", "SECTORS",
		"REJECT", "BLOCKMAP", "MAP17", "THINGS", "LINEDEFS", "SIDEDEFS",
		"VERTEXES", "SEGS", "SSECTORS", "NODES", "SECTORS", "REJECT",
		"BLOCKMAP", "MAP18", "THINGS", "LINEDEFS", "SIDEDEFS", "VERTEXES",
		"SEGS", "SSECTORS", "NODES", "SECTORS", "REJECT", "BLOCKMAP",
		"MAP19", "THINGS", "LINEDEFS", "SIDEDEFS", "VERTEXES", "SEGS",
		"SSECTORS", "NODES", "SECTORS", "REJECT", "BLOCKMAP", "MAP20",
		"THINGS", "LINEDEFS", "SIDEDEFS", "VERTEXES", "SEGS", "SSECTORS",
		"NODES", "SECTORS", "REJECT", "BLOCKMAP", "MAP21", "THINGS",
		"LINEDEFS", "SIDEDEFS", "VERTEXES", "SEGS", "SSECTORS", "NODES",
		"SECTORS", "REJECT", "BLOCKMAP", "MAP22", "THINGS", "LINEDEFS",
		"SIDEDEFS", "VERTEXES", "SEGS", "SSECTORS", "NODES", "SECTORS",
		"REJECT", "BLOCKMAP", "MAP23", "THINGS", "LINEDEFS", "SIDEDEFS",
		"VERTEXES", "SECTORS", "MAP24", "THINGS", "LINEDEFS", "SIDEDEFS",
		"VERTEXES", "SECTORS",
		];

		assert_eq!(wad.lumps.len(), lump_names.len());
		let lumps_and_names = wad.lumps.iter().zip(lump_names);
		for lump_and_name in lumps_and_names {
			match lump_and_name {
				(ref lump, name) => {
					let name = String::from(name);
					assert_eq!(lump.name, name);
				}
			}
		}
		Ok(())
	}

	#[test]
	fn can_write_wad() -> Result<(), Box<dyn Error>> {
		let wad = DoomWad::load("tests/data/3difytest.wad")?;
		wad.write("tests/data/another.wad")?;

		let wad_file = File::open("tests/data/3difytest.wad")?;
		let out_wad_file = File::open("tests/data/another.wad")?;
		wad_file.bytes().zip(out_wad_file.bytes())
			.all(|bytes| bytes.0.unwrap() == bytes.1.unwrap());
		std::fs::remove_file("tests/data/another.wad")?;
		Ok(())
	}
}

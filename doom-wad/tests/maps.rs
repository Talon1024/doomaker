#[cfg(test)]
mod tests {
    use std::{error::Error, fmt::Display};
    use doom_wad::wad::{DoomWad, LumpName};
    use futures::executor;

    #[derive(Debug)]
    struct MapNotFoundError(LumpName);

    impl Display for MapNotFoundError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Map {} not found!", self.0.as_str())
        }
    }

    impl Error for MapNotFoundError {}

    #[test]
    fn can_find_all_maps_vanilla() -> Result<(), Box<dyn Error>> {
        let filename = "tests/data/3difytest.wad";
        let wad = executor::block_on(DoomWad::load(filename))?;
        let actual_maps: Vec<_> = wad.lumps.iter()
            .map(|l| l.name)
            .filter(|&n| doom_wad::map::is_map(n))
            .collect();

        let expected_maps = [
            LumpName::try_from("MAP01").unwrap(),
            LumpName::try_from("MAP02").unwrap(),
            LumpName::try_from("MAP03").unwrap(),
            LumpName::try_from("MAP04").unwrap(),
            LumpName::try_from("MAP05").unwrap(),
            LumpName::try_from("MAP06").unwrap(),
            LumpName::try_from("MAP07").unwrap(),
            LumpName::try_from("MAP08").unwrap(),
            LumpName::try_from("MAP09").unwrap(),
            LumpName::try_from("MAP10").unwrap(),
            LumpName::try_from("MAP11").unwrap(),
            LumpName::try_from("MAP12").unwrap(),
            LumpName::try_from("MAP13").unwrap(),
            LumpName::try_from("MAP14").unwrap(),
            LumpName::try_from("MAP15").unwrap(),
            LumpName::try_from("MAP16").unwrap(),
            LumpName::try_from("MAP17").unwrap(),
            LumpName::try_from("MAP18").unwrap(),
            LumpName::try_from("MAP19").unwrap(),
            LumpName::try_from("MAP20").unwrap(),
            LumpName::try_from("MAP21").unwrap(),
            LumpName::try_from("MAP22").unwrap(),
            LumpName::try_from("MAP23").unwrap(),
            LumpName::try_from("MAP24").unwrap(),
        ];

        assert_eq!(expected_maps.len(), actual_maps.len());

        expected_maps.iter().copied()
            .map(|map_name| {
                match actual_maps.contains(&map_name) {
                    true => Ok(()),
                    false => Err(Box::from(MapNotFoundError(map_name))),
                }
            })
            .collect()
    }

    #[test]
    fn can_find_all_maps_versatile() -> Result<(), Box<dyn Error>> {
        let filename = "tests/data/3difytest.wad";
        let wad = executor::block_on(DoomWad::load(filename))?;
        let actual_maps = doom_wad::map::find_maps(&wad, None);
        let map_names: Vec<_> = actual_maps.iter().map(|map| map.name).collect();

        let expected_maps = [
            LumpName::try_from("MAP01").unwrap(),
            LumpName::try_from("MAP02").unwrap(),
            LumpName::try_from("MAP03").unwrap(),
            LumpName::try_from("MAP04").unwrap(),
            LumpName::try_from("MAP05").unwrap(),
            LumpName::try_from("MAP06").unwrap(),
            LumpName::try_from("MAP07").unwrap(),
            LumpName::try_from("MAP08").unwrap(),
            LumpName::try_from("MAP09").unwrap(),
            LumpName::try_from("MAP10").unwrap(),
            LumpName::try_from("MAP11").unwrap(),
            LumpName::try_from("MAP12").unwrap(),
            LumpName::try_from("MAP13").unwrap(),
            LumpName::try_from("MAP14").unwrap(),
            LumpName::try_from("MAP15").unwrap(),
            LumpName::try_from("MAP16").unwrap(),
            LumpName::try_from("MAP17").unwrap(),
            LumpName::try_from("MAP18").unwrap(),
            LumpName::try_from("MAP19").unwrap(),
            LumpName::try_from("MAP20").unwrap(),
            LumpName::try_from("MAP21").unwrap(),
            LumpName::try_from("MAP22").unwrap(),
            LumpName::try_from("MAP23").unwrap(),
            LumpName::try_from("MAP24").unwrap(),
        ];

        assert_eq!(expected_maps.len(), actual_maps.len());

        expected_maps.iter().copied()
            .map(|map_name| {
                match map_names.contains(&map_name) {
                    true => Ok(()),
                    false => Err(Box::from(MapNotFoundError(map_name))),
                }
            })
            .collect()
    }
}

/// The parser for UDMF-format Doom maps
pub mod parser {
    use pest_derive::Parser;
    #[derive(Debug, Parser)]
    #[grammar = "pest-grammars/common.pest"]
    #[grammar = "pest-grammars/udmf.pest"]
    pub struct UDMFParser;
}

/// Input types, used to parse the tokens into UDMF data
pub mod input {
    use std::{collections::HashMap, str::FromStr, error::Error};
    use ahash::RandomState;
    use pest::Parser;
    use thiserror::Error;
    use parse_display::Display;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Display, Default)]
    pub enum UDMFObjectType {
        #[default]
        Unknown,
        Thing,
        Linedef,
        Sidedef,
        Sector,
        Vertex,
    }

    // For getting properties which have "no valid default"
    macro_rules! get_property_req {
        ($map: ident, $key: ident) => {
            // NOTE: $map must be mutable
            let $key = $map.data.remove(stringify!($key))
                .ok_or(UDMFError::RequiredKeyNotFound {
                    key: String::from(stringify!($key))
                })?;
            let $key = $key.parse().map_err(|e| {
                let orig_error = Box::from(e);
                UDMFError::DatumConversionFailed {
                    key: String::from(stringify!($key)),
                    datum: $key,
                    orig_error
                }
            })?;
        };
        // In case the key is a Rust keyword
        ($map: ident, $key: ident, $alias: ident) => {
            let $alias = $map.data.remove(stringify!($key))
                .ok_or(UDMFError::RequiredKeyNotFound {
                    key: String::from(stringify!($key))
                })?;
            // The output type will usually be inferred from how it is used
            let $alias = $alias.parse().map_err(|e| {
                let orig_error = Box::from(e);
                UDMFError::DatumConversionFailed {
                    key: String::from(stringify!($key)),
                    datum: $alias,
                    orig_error
                }
            })?;
        };
    }

    // For properties which are often present, but have default values
    macro_rules! get_property_opt {
        ($map: ident, $key: ident) => {
            let $key = $map.data.remove(stringify!($key)).map(|v| {
                v.parse().map_err(|e| {
                    let orig_error = Box::from(e);
                    UDMFError::DatumConversionFailed {
                        key: String::from(stringify!($key)),
                        datum: v,
                        orig_error
                    }
                })
            })
            .unwrap_or(Ok(Default::default()))?;
        };
    }

    pub type PropMap = HashMap<String, String, RandomState>;

    #[derive(Debug, Clone, Default)]
    struct UDMFObject {
        object_type: UDMFObjectType,
        data: PropMap,
    }

    #[derive(Debug, Error)]
    pub enum UDMFError {
        /// A key in the UDMF object property dictionary was not found
        #[error("Required key {key} not found!")]
        RequiredKeyNotFound { key: String },
        /// The value could not be converted to the built-in value
        #[error("Could not convert datum ({key}: {datum})\n{orig_error}")]
        DatumConversionFailed {
            key: String,
            datum: String,
            orig_error: Box<dyn Error>
        },
        /// The incorrect UDMF object type was given
        #[error("UDMF Object type mismatch! Expected {expected}, got {object_is}")]
        ObjectTypeMismatch { object_is: UDMFObjectType, expected: UDMFObjectType },
        #[error("Could not parse the TEXTMAP\n{orig_error}")]
        TextMapParseError { orig_error: Box<dyn Error> },
        #[error("Unknown UDMF object type {object_is}")]
        UnknownObjectType { object_is: String }
    }

    trait OptionalUDMFData : PartialEq + Eq + Default {
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

    impl OptionalUDMFData for SidedefTexture {}

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

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    // Default can just be derived here, since the numbers for additive colours
    // are supposed to be 0 by default
    pub struct AdditiveColour {
        pub r: u8,
        pub g: u8,
        pub b: u8,
    }

    macro_rules! impl_from_str_for_colour {
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
        )*};
    }
    impl_from_str_for_colour! { MultiplicativeColour AdditiveColour }

    trait UDMFOutput {
        const UDMF_OBJECT_TYPE: UDMFObjectType;
    }

    #[derive(Debug, Clone)]
    pub struct UDMFThing {
        pub x: f32,
        pub y: f32,
        pub height: f32,
        pub angle: i32,
        pub ednum: u32,
        pub id: u32,
        pub props: PropMap,
    }

    impl UDMFOutput for UDMFThing {
        const UDMF_OBJECT_TYPE: UDMFObjectType = UDMFObjectType::Thing;
    }

    impl TryFrom<UDMFObject> for UDMFThing {
        type Error = UDMFError;

        fn try_from(mut value: UDMFObject) -> Result<Self, Self::Error> {
            if value.object_type != Self::UDMF_OBJECT_TYPE {
                Err(UDMFError::ObjectTypeMismatch {
                    object_is: value.object_type,
                    expected: Self::UDMF_OBJECT_TYPE
                })
            } else {
                get_property_req!(value, x);
                get_property_req!(value, y);
                get_property_req!(value, type, ednum);
                get_property_opt!(value, height);
                get_property_opt!(value, angle);
                get_property_opt!(value, id);
                Ok(UDMFThing {
                    x,
                    y,
                    ednum,
                    height,
                    angle,
                    id,
                    props: value.data
                })
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct UDMFLinedef {
        pub v1: u32,
        pub v2: u32,
        pub id: u32,
        pub sidefront: u32,
        pub sideback: SidedefIndex,
        pub props: PropMap,
    }

    impl UDMFOutput for UDMFLinedef {
        const UDMF_OBJECT_TYPE: UDMFObjectType = UDMFObjectType::Linedef;
    }

    impl TryFrom<UDMFObject> for UDMFLinedef {
        type Error = UDMFError;

        fn try_from(mut value: UDMFObject) -> Result<Self, Self::Error> {
            if value.object_type != Self::UDMF_OBJECT_TYPE {
                Err(UDMFError::ObjectTypeMismatch {
                    object_is: value.object_type,
                    expected: Self::UDMF_OBJECT_TYPE
                })
            } else {
                get_property_req!(value, v1);
                get_property_req!(value, v2);
                get_property_req!(value, sidefront);
                get_property_opt!(value, id);
                get_property_opt!(value, sideback);
                Ok(UDMFLinedef {
                    v1,
                    v2,
                    id,
                    sidefront,
                    sideback,
                    props: value.data
                })
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct UDMFSidedef {
        pub sector: u32,
        pub offsetx: i32,
        pub offsety: i32,
        pub texturetop: SidedefTexture,
        pub texturemiddle: SidedefTexture,
        pub texturebottom: SidedefTexture,
        pub props: PropMap,
    }

    impl UDMFOutput for UDMFSidedef {
        const UDMF_OBJECT_TYPE: UDMFObjectType = UDMFObjectType::Sidedef;
    }

    impl TryFrom<UDMFObject> for UDMFSidedef {
        type Error = UDMFError;

        fn try_from(mut value: UDMFObject) -> Result<Self, Self::Error> {
            if value.object_type != Self::UDMF_OBJECT_TYPE {
                Err(UDMFError::ObjectTypeMismatch {
                    object_is: value.object_type,
                    expected: Self::UDMF_OBJECT_TYPE
                })
            } else {
                get_property_req!(value, sector);
                get_property_opt!(value, offsetx);
                get_property_opt!(value, offsety);
                get_property_opt!(value, texturetop);
                get_property_opt!(value, texturemiddle);
                get_property_opt!(value, texturebottom);
                Ok(UDMFSidedef {
                    sector,
                    offsetx,
                    offsety,
                    texturetop,
                    texturemiddle,
                    texturebottom,
                    props: value.data
                })
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct UDMFSector {
        pub texturefloor: String,
        pub textureceiling: String,
        pub heightfloor: i32,
        pub heightceiling: i32,
        pub lightlevel: LightLevel,
        pub special: u32,
        pub id: u32,
        pub color_sprites: MultiplicativeColour,
        pub color_walltop: MultiplicativeColour,
        pub color_ceiling: MultiplicativeColour,
        pub color_floor: MultiplicativeColour,
        pub color_wallbottom: MultiplicativeColour,
        pub props: PropMap,
    }

    impl UDMFOutput for UDMFSector {
        const UDMF_OBJECT_TYPE: UDMFObjectType = UDMFObjectType::Sector;
    }

    impl TryFrom<UDMFObject> for UDMFSector {
        type Error = UDMFError;

        fn try_from(mut value: UDMFObject) -> Result<Self, Self::Error> {
            if value.object_type != Self::UDMF_OBJECT_TYPE {
                Err(UDMFError::ObjectTypeMismatch {
                    object_is: value.object_type,
                    expected: Self::UDMF_OBJECT_TYPE
                })
            } else {
                get_property_req!(value, texturefloor);
                get_property_req!(value, textureceiling);
                get_property_opt!(value, heightfloor);
                get_property_opt!(value, heightceiling);
                get_property_opt!(value, lightlevel);
                get_property_opt!(value, id);
                get_property_opt!(value, special);
                get_property_opt!(value, color_sprites);
                get_property_opt!(value, color_walltop);
                get_property_opt!(value, color_ceiling);
                get_property_opt!(value, color_floor);
                get_property_opt!(value, color_wallbottom);
                Ok(UDMFSector {
                    texturefloor,
                    textureceiling,
                    heightfloor,
                    heightceiling,
                    lightlevel,
                    id,
                    special,
                    color_sprites,
                    color_walltop,
                    color_ceiling,
                    color_floor,
                    color_wallbottom,
                    props: value.data
                })
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct UDMFVertex {
        pub x: f32,
        pub y: f32,
        pub props: PropMap,
    }

    impl UDMFOutput for UDMFVertex {
        const UDMF_OBJECT_TYPE: UDMFObjectType = UDMFObjectType::Vertex;
    }

    impl TryFrom<UDMFObject> for UDMFVertex {
        type Error = UDMFError;

        fn try_from(mut value: UDMFObject) -> Result<Self, Self::Error> {
            if value.object_type != Self::UDMF_OBJECT_TYPE {
                Err(UDMFError::ObjectTypeMismatch {
                    object_is: value.object_type,
                    expected: Self::UDMF_OBJECT_TYPE
                })
            } else {
                get_property_req!(value, x);
                get_property_req!(value, y);
                Ok(UDMFVertex { x, y, props: value.data })
            }
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct UDMFMap {
        pub namespace: String,
        pub things: Vec<UDMFThing>,
        pub linedefs: Vec<UDMFLinedef>,
        pub sidedefs: Vec<UDMFSidedef>,
        pub vertices: Vec<UDMFVertex>,
        pub sectors: Vec<UDMFSector>,
    }

    impl FromStr for UDMFMap {
        type Err = UDMFError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            use super::parser::{Rule, UDMFParser};
            let start = Rule::udmf_map;
            let mut root = UDMFParser::parse(start, s).map_err(|e| {
                let orig_error = Box::from(e);
                UDMFError::TextMapParseError { orig_error }
            })?;

            let mut map = UDMFMap::default();
            root.try_for_each(|token| {
                match token.as_rule() {
                    Rule::namespace => {
                        let token = token.into_inner().next().unwrap();
                        match token.as_rule() {
                            Rule::text_piece => {
                                map.namespace.push_str(token.as_str());
                            },
                            unknown => unreachable!("Rule: {unknown:?}"),
                        }
                    },
                    Rule::data_block => {
                        let mut object = UDMFObject::default();
                        let mut object_is = String::new();
                        token.into_inner().for_each(|token| {
                            match token.as_rule() {
                                Rule::udmf_object_type => {
                                    object.object_type = match token.as_str() {
                                        "thing" => UDMFObjectType::Thing,
                                        "linedef" => UDMFObjectType::Linedef,
                                        "sidedef" => UDMFObjectType::Sidedef,
                                        "sector" => UDMFObjectType::Sector,
                                        "vertex" => UDMFObjectType::Vertex,
                                        unknown => {
                                            object_is.push_str(unknown);
                                            UDMFObjectType::Unknown
                                        },
                                    };
                                },
                                Rule::key_value_pair => {
                                    let mut key = String::default();
                                    let mut value = String::default();
                                    token.into_inner().for_each(|token| {
                                        match token.as_rule() {
                                            Rule::identifier => {
                                                key.push_str(token.as_str());
                                            },
                                            Rule::any_data => {
                                                value.push_str(token.as_str());
                                            },
                                            unknown => unreachable!("Rule: {unknown:?}"),
                                        }
                                    });
                                    object.data.insert(key, value);
                                }
                                unknown => unreachable!("Rule: {unknown:?}"),
                            }
                        });
                        match object.object_type {
                            UDMFObjectType::Unknown => {
                                return Err(UDMFError::UnknownObjectType { object_is });
                            },
                            UDMFObjectType::Thing => {
                                let value = UDMFThing::try_from(object)?;
                                map.things.push(value);
                            },
                            UDMFObjectType::Linedef => {
                                let value = UDMFLinedef::try_from(object)?;
                                map.linedefs.push(value);
                            },
                            UDMFObjectType::Sidedef => {
                                let value = UDMFSidedef::try_from(object)?;
                                map.sidedefs.push(value);
                            },
                            UDMFObjectType::Sector => {
                                let value = UDMFSector::try_from(object)?;
                                map.sectors.push(value);
                            },
                            UDMFObjectType::Vertex => {
                                let value = UDMFVertex::try_from(object)?;
                                map.vertices.push(value);
                            },
                        }
                    }
                    unknown => unreachable!("Rule: {unknown:?}"),
                }
                Ok(())
            })?;
            Ok(map)
        }
    }
}
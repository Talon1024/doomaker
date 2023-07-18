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
    use std::collections::HashMap;
    use ahash::RandomState;
    use thiserror::Error;
    use parse_display::Display;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
    pub enum UDMFObjectType {
        Thing,
        Linedef,
        Sidedef,
        Sector,
        Vertex,
    }

    macro_rules! get_property {
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

    pub type PropMap = HashMap<String, String, RandomState>;

    #[derive(Debug, Clone)]
    struct UDMFObject {
        object_type: UDMFObjectType,
        data: PropMap,
    }

    #[derive(Debug, Error)]
    pub enum UDMFError {
        #[error("Required key {key} not found!")]
        RequiredKeyNotFound { key: String },
        #[error("Could not convert datum ({key}: {datum})\n{orig_error}")]
        DatumConversionFailed {
            key: String,
            datum: String,
            orig_error: Box<dyn std::error::Error>
        },
        #[error("UDMF Object type mismatch! Expected {expected}, got {object_is}")]
        ObjectTypeMismatch { object_is: UDMFObjectType, expected: UDMFObjectType }
    }

    trait UDMFOutput {
        const UDMF_OBJECT_TYPE: UDMFObjectType;
    }

    #[derive(Debug, Clone)]
    pub struct UDMFThing {
        pub x: f32,
        pub y: f32,
        pub ednum: u32,
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
                get_property!(value, x);
                get_property!(value, y);
                get_property!(value, type, ednum);
                Ok(UDMFThing { x, y, ednum, props: value.data })
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct UDMFLinedef {
        pub v1: u32,
        pub v2: u32,
        pub sidefront: u32,
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
                get_property!(value, v1);
                get_property!(value, v2);
                get_property!(value, sidefront);
                Ok(UDMFLinedef { v1, v2, sidefront, props: value.data })
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct UDMFSidedef {
        pub sector: u32,
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
                get_property!(value, sector);
                Ok(UDMFSidedef { sector, props: value.data })
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct UDMFSector {
        pub texturefloor: String,
        pub textureceiling: String,
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
                get_property!(value, texturefloor);
                get_property!(value, textureceiling);
                Ok(UDMFSector { texturefloor, textureceiling, props: value.data })
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
                get_property!(value, x);
                get_property!(value, y);
                Ok(UDMFVertex { x, y, props: value.data })
            }
        }
    }

    pub struct UDMFMap {
        pub map_namespace: String,
        pub things: Vec<UDMFThing>,
        pub linedefs: Vec<UDMFLinedef>,
        pub sidedefs: Vec<UDMFSidedef>,
        pub vertices: Vec<UDMFVertex>,
        pub sectors: Vec<UDMFSector>,
    }
}
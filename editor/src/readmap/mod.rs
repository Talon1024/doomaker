use std::num::NonZeroU32;

use glam::Vec2;
use map_to_3D::plane::Plane;
use crate::{
    data::{Vertex, Line, Side, Sector, Thing},
    data::property::{Properties, PropertyValue}
};
use doom_wad::map::{
    Vertex as DVVertex,
    Linedef as DVLine,
    Sidedef as DVSide,
    Sector as DVSector,
    Thing as DVThing,
    LinedefFlags,
};

impl From<DVVertex> for Vertex {
    fn from(v: DVVertex) -> Self {
        Vertex::new(Vec2::from([v.x as f32, v.y as f32]))
    }
}

macro_rules! non_negative_integer {
    ($in: expr, $ty: ty) => {
        match $in {
            <$ty>::MAX => None,
            v => Some(v.into())
        }
    };
}

impl From<DVLine> for Line {
    fn from(v: DVLine) -> Self {
        Line {
            id: None,
            v1: v.v1.into(),
            v2: v.v2.into(),
            blocking: v.flags.contains(LinedefFlags::BLOCK_PLAYERS),
            blockmonsters: v.flags.contains(LinedefFlags::BLOCK_MONSTERS),
            twosided: v.flags.contains(LinedefFlags::TWO_SIDED),
            dontpegtop: v.flags.contains(LinedefFlags::UPPER_UNPEGGED),
            dontpegbottom: v.flags.contains(LinedefFlags::LOWER_UNPEGGED),
            secret: v.flags.contains(LinedefFlags::AUTOMAP_SOLID),
            blocksound: v.flags.contains(LinedefFlags::BLOCK_SOUND),
            dontdraw: v.flags.contains(LinedefFlags::AUTOMAP_HIDDEN),
            mapped: v.flags.contains(LinedefFlags::AUTOMAP_SHOWN),
            passuse: false,
            translucent: false,
            jumpover: false,
            blockfloaters: false,
            playercross: false,
            playeruse: false,
            monstercross: false,
            monsteruse: false,
            impact: false,
            playerpush: false,
            monsterpush: false,
            missilecross: false,
            repeatspecial: false,
            special: v.special.into(),
            arg0: v.tag.into(),
            arg1: 0,
            arg2: 0,
            arg3: 0,
            arg4: 0,
            sidefront: v.front.into(),
            sideback: non_negative_integer!(v.back, u16),
            comment: Default::default(),
            properties: Default::default(),
        }
    }
}

impl From<DVSide> for Side {
    fn from(v: DVSide) -> Self {
        let upper = String::from(String::from_utf8_lossy(&v.upper));
        let middle = String::from(String::from_utf8_lossy(&v.middle));
        let lower = String::from(String::from_utf8_lossy(&v.lower));
        let mut side = Side::new(upper, middle, lower, v.sec as usize);
        side.set_property("offsetx", Some(PropertyValue::Integer(v.x as i32)));
        side.set_property("offsety", Some(PropertyValue::Integer(v.y as i32)));
        side
    }
}

impl From<DVSector> for Sector {
    fn from(v: DVSector) -> Self {
        let florp = Plane::Flat(v.florh as f32);
        let ceilp = Plane::Flat(v.ceilh as f32);
        let florm = String::from(String::from_utf8_lossy(&v.flort));
        let ceilm = String::from(String::from_utf8_lossy(&v.ceilt));
        let mut sector = Sector::new(florp, ceilp, florm, ceilm);
        sector.set_property("lightlevel", Some(PropertyValue::Integer(v.light as i32)));
        sector.set_property("special", Some(PropertyValue::Integer(v.special as i32)));
        sector.set_property("tag", Some(PropertyValue::Integer(v.tag as i32)));
        sector
    }
}

impl From<DVThing> for Thing {
    fn from(v: DVThing) -> Self {
        let /* mut */ thing = Thing::new(Vec2::new(v.x as f32, v.y as f32), 0., v.angle as f32, v.ednum as usize);
        // TODO: Flags to properties
        thing
    }
}

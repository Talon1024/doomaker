use glam::Vec2;
use map_to_3D::plane::Plane;
use crate::{
	data::{Vertex, Line, Side, Sector, Thing},
	property::{Properties, PropertyValue}
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

impl From<DVLine> for Line {
	fn from(v: DVLine) -> Self {
		let mut line = Line::new(
			v.a as usize,
			v.b as usize,
			v.front as usize,
			match v.back {
				u16::MAX => None,
				v => Some(v as usize)
			}
		);
		if v.special != 0 {
			line.set_property("special", Some(PropertyValue::UnsignedInteger(v.special as u32)));
		}
		if v.tag != 0 {
			line.set_property("tag", Some(PropertyValue::UnsignedInteger(v.tag as u32)));
		}
		// Flags
		if !(v.flags & LinedefFlags::BLOCK_PLAYERS).is_empty() {
			line.set_property("blocking", Some(PropertyValue::Boolean(true)));
		}
		if !(v.flags & LinedefFlags::BLOCK_MONSTERS).is_empty() {
			line.set_property("blockmonsters", Some(PropertyValue::Boolean(true)));
		}
		if !(v.flags & LinedefFlags::TWO_SIDED).is_empty() {
			line.set_property("twosided", Some(PropertyValue::Boolean(true)));
		}
		if !(v.flags & LinedefFlags::UPPER_UNPEGGED).is_empty() {
			line.set_property("dontpegtop", Some(PropertyValue::Boolean(true)));
		}
		if !(v.flags & LinedefFlags::LOWER_UNPEGGED).is_empty() {
			line.set_property("dontpegbottom", Some(PropertyValue::Boolean(true)));
		}
		if !(v.flags & LinedefFlags::AUTOMAP_SOLID).is_empty() {
			line.set_property("secret", Some(PropertyValue::Boolean(true)));
		}
		if !(v.flags & LinedefFlags::BLOCK_SOUND).is_empty() {
			line.set_property("blocksound", Some(PropertyValue::Boolean(true)));
		}
		if !(v.flags & LinedefFlags::AUTOMAP_HIDDEN).is_empty() {
			line.set_property("dontdraw", Some(PropertyValue::Boolean(true)));
		}
		if !(v.flags & LinedefFlags::AUTOMAP_SHOWN).is_empty() {
			line.set_property("mapped", Some(PropertyValue::Boolean(true)));
		}
		line
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

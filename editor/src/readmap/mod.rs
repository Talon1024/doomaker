use glam::Vec2;
use crate::{data::{Vertex, Line}, property::{Properties, PropertyValue}};
use doomwad::map::{
	Vertex as DVVertex,
	Linedef as DVLine,
	Sidedef as DVSide,
	Sector as DVSector,
	Thing as DVThing,
	linedef_flags::*
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
			v.back as usize
		);
		if v.special != 0 {
			line.set_property("special", Some(PropertyValue::UnsignedInteger(v.special as u32)));
		}
		if v.tag != 0 {
			line.set_property("tag", Some(PropertyValue::UnsignedInteger(v.tag as u32)));
		}
		// Flags
		if (v.flags & LF_BLOCK_PLAYERS) != 0 {
			line.set_property("blocking", Some(PropertyValue::Boolean(true)));
		}
		if (v.flags & LF_BLOCK_MONSTERS) != 0 {
			line.set_property("blockmonsters", Some(PropertyValue::Boolean(true)));
		}
		if (v.flags & LF_TWO_SIDED) != 0 {
			line.set_property("twosided", Some(PropertyValue::Boolean(true)));
		}
		if (v.flags & LF_UPPER_UNPEGGED) != 0 {
			line.set_property("dontpegtop", Some(PropertyValue::Boolean(true)));
		}
		if (v.flags & LF_LOWER_UNPEGGED) != 0 {
			line.set_property("dontpegbottom", Some(PropertyValue::Boolean(true)));
		}
		if (v.flags & LF_AUTOMAP_SOLID) != 0 {
			line.set_property("secret", Some(PropertyValue::Boolean(true)));
		}
		if (v.flags & LF_BLOCK_SOUND) != 0 {
			line.set_property("blocksound", Some(PropertyValue::Boolean(true)));
		}
		if (v.flags & LF_AUTOMAP_HIDDEN) != 0 {
			line.set_property("dontdraw", Some(PropertyValue::Boolean(true)));
		}
		if (v.flags & LF_AUTOMAP_SHOWN) != 0 {
			line.set_property("mapped", Some(PropertyValue::Boolean(true)));
		}
		line
	}
}

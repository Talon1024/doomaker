use std::collections::HashMap;
use ahash::RandomState;
use glam::Vec2;
use map_to_3D::plane::Plane;
use crate::property::{Properties, PropertyValue as PVal};

#[derive(Default, Clone, Debug)]
pub(crate) struct Vertex {
	pub xy: Vec2,
	properties: HashMap<String, PVal, RandomState>,
}

impl Vertex {
	pub fn new(xy: Vec2) -> Vertex {
		Vertex { xy, ..Default::default() }
	}
}

// I could probably generate these with a custom macro, but I don't know a lot
// about Rust's macros to write a custom one to do this.
impl Properties for Vertex {
	fn set_property(&mut self, prop: &str, value: Option<PVal>) {
		if let Some(value) = value {
			match prop {
				"xy" => if let PVal::Vec2(v) = value {
					self.xy = v
				},
				"x" => if let PVal::Float(v) = value {
					self.xy.x = v
				},
				"y" => if let PVal::Float(v) = value {
					self.xy.y = v
				},
				// insert and remove return values
				_ => {self.properties.insert(prop.to_string(), value);}
			}
		} else {
			match prop {
				// These properties MUST be set, so do nothing.
				"xy" => (),
				"x" => (),
				"y" => (),
				_ => {self.properties.remove(prop);},
			}
		}
	}
	fn get_property(&self, prop: &str) -> Option<PVal> {
		match prop {
			"xy" => Some(PVal::Vec2(self.xy)),
			"x" => Some(PVal::Float(self.xy.x)),
			"y" => Some(PVal::Float(self.xy.y)),
			_ => self.properties.get(prop).cloned()
		}
	}
}

#[derive(Default, Clone, Debug)]
pub(crate) struct Line {
	/// Vertex A
	pub va: usize,
	/// Vertex B
	pub vb: usize,
	/// Front side
	pub sf: usize,
	/// Back side
	pub sb: Option<usize>,
	properties: HashMap<String, PVal, RandomState>,
}

impl Line {
	pub fn new(va: usize, vb: usize, sf: usize, sb: Option<usize>) -> Line {
		Line {va, vb, sf, sb, ..Default::default()}
	}
}

impl Properties for Line {
	fn set_property(&mut self, prop: &str, value: Option<PVal>) {
		if let Some(value) = value {
			match prop {
				"va" => if let PVal::Index(v) = value {
					self.va = v
				},
				"vb" => if let PVal::Index(v) = value {
					self.vb = v
				},
				"sf" => if let PVal::Index(v) = value {
					self.sf = v
				},
				"sb" => if let PVal::Index(v) = value {
					self.sb = Some(v)
				}
				_ => {self.properties.insert(prop.to_string(), value);}
			}
		} else {
			match prop {
				"va" => (),
				"vb" => (),
				"sf" => (),
				"sb" => self.sb = None,
				_ => {self.properties.remove(prop);},
			}
		}
	}
	fn get_property(&self, prop: &str) -> Option<PVal> {
		match prop {
			"va" => Some(PVal::Index(self.va)),
			"vb" => Some(PVal::Index(self.vb)),
			"sf" => Some(PVal::Index(self.sf)),
			"sb" => self.sb.map(PVal::Index),
			_ => self.properties.get(prop).cloned()
		}
	}
}

#[derive(Default, Clone, Debug)]
pub(crate) struct Side {
	pub mtl_upper: String,
	pub mtl_middle: String,
	pub mtl_lower: String,
	pub sector: usize,
	properties: HashMap<String, PVal, RandomState>,
}

impl Side {
	pub fn new(mtl_upper: String, mtl_middle: String, mtl_lower: String, sector: usize) -> Side {
		Side { mtl_upper, mtl_middle, mtl_lower, sector, ..Default::default() }
	}
}

impl Properties for Side {
	fn set_property(&mut self, prop: &str, value: Option<PVal>) {
		if let Some(value) = value {
			match prop {
				"mtl_upper" => if let PVal::String(v) = value {
					self.mtl_upper = v
				},
				"mtl_middle" => if let PVal::String(v) = value {
					self.mtl_middle = v
				},
				"mtl_lower" => if let PVal::String(v) = value {
					self.mtl_lower = v
				},
				"sector" => if let PVal::Index(v) = value {
					self.sector = v
				}
				_ => {self.properties.insert(prop.to_string(), value);}
			}
		} else {
			match prop {
				"mtl_upper" => {self.mtl_upper = String::from("-");},
				"mtl_middle" => {self.mtl_middle = String::from("-");},
				"mtl_lower" => {self.mtl_lower = String::from("-");},
				"sector" => (),
				_ => {self.properties.remove(prop);},
			}
		}
	}
	fn get_property(&self, prop: &str) -> Option<PVal> {
		match prop {
			"mtl_upper" => Some(PVal::String(self.mtl_upper.clone())),
			"mtl_middle" => Some(PVal::String(self.mtl_middle.clone())),
			"mtl_lower" => Some(PVal::String(self.mtl_lower.clone())),
			"sector" => Some(PVal::Index(self.sector)),
			_ => self.properties.get(prop).cloned()
		}
	}
}

#[derive(Default, Clone, Debug)]
pub(crate) struct Sector {
	pub plane_floor: Plane,
	pub plane_ceil: Plane,
	pub mtl_floor: String,
	pub mtl_ceil: String,
	properties: HashMap<String, PVal, RandomState>,
}

impl Sector {
	pub fn new(floor: Plane, ceil: Plane, mtl_floor: String, mtl_ceil: String) -> Sector {
		Sector { plane_floor: floor, plane_ceil: ceil, mtl_floor, mtl_ceil, ..Default::default() }
	}
}

impl Properties for Sector {
	fn set_property(&mut self, prop: &str, value: Option<PVal>) {
		if let Some(value) = value {
			match prop {
				"mtl_floor" => if let PVal::String(v) = value {
					self.mtl_floor = v
				},
				"mtl_ceil" => if let PVal::String(v) = value {
					self.mtl_ceil = v
				},
				_ => {self.properties.insert(prop.to_string(), value);}
			}
		} else {
			match prop {
				"mtl_floor" => {self.mtl_floor = String::from("-");},
				"mtl_ceil" => {self.mtl_ceil = String::from("-");},
				_ => {self.properties.remove(prop);},
			}
		}
	}
	fn get_property(&self, prop: &str) -> Option<PVal> {
		match prop {
			"mtl_floor" => Some(PVal::String(self.mtl_floor.clone())),
			"mtl_ceil" => Some(PVal::String(self.mtl_ceil.clone())),
			_ => self.properties.get(prop).cloned()
		}
	}
}

#[derive(Default, Clone, Debug)]
pub(crate) struct Thing {
	pub pos: Vec2,
	pub z: f32,
	pub angle: f32,
	pub typeid: usize,
	properties: HashMap<String, PVal, RandomState>,
}

impl Thing {
	pub fn new(pos: Vec2, z: f32, angle: f32, typeid: usize) -> Thing {
		Thing { pos, z, typeid, ..Default::default() }
	}
}

impl Properties for Thing {
	fn set_property(&mut self, prop: &str, value: Option<PVal>) {
		if let Some(value) = value {
			match prop {
				"pos" => if let PVal::Vec2(v) = value {
					self.pos = v
				},
				"z" => if let PVal::Float(v) = value {
					self.z = v
				},
				"angle" => if let PVal::Float(v) = value {
					self.angle = v
				}
				"typeid" => if let PVal::Index(v) = value {
					self.typeid = v
				},
				_ => {self.properties.insert(prop.to_string(), value);}
			}
		} else {
			match prop {
				"pos" => (),
				"z" => {self.z = 0.;}, // Default
				"angle" => (),
				"typeid" => (),
				_ => {self.properties.remove(prop);},
			}
		}
	}
	fn get_property(&self, prop: &str) -> Option<PVal> {
		match prop {
			"pos" => Some(PVal::Vec2(self.pos)),
			"z" => Some(PVal::Float(self.z)),
			"angle" => Some(PVal::Float(self.angle)),
			"typeid" => Some(PVal::Index(self.typeid)),
			_ => self.properties.get(prop).cloned()
		}
	}
}

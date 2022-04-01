use super::{SectorPolygon, Edge, Vector2};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl SectorPolygon {
	#[wasm_bindgen(getter, readonly)]
	pub fn vertices(&self) -> Box<[usize]> {
		Box::from(&self.vertices[..])
	}
}

#[wasm_bindgen]
pub struct SectorPolygons {
	polys: Vec<SectorPolygon>,
	cur: usize,
}

#[wasm_bindgen]
pub struct SectorPolygonIteration {
	#[wasm_bindgen(getter_with_clone)]
	pub value: Option<SectorPolygon>,
	pub done: bool
}

#[wasm_bindgen]
impl SectorPolygons {
	pub fn next(&mut self) -> SectorPolygonIteration {
		SectorPolygonIteration {
			value: self.polys.get(self.cur).cloned(),
			done: self.cur >= self.polys.len()
		}
	}
}

impl SectorPolygons {
	fn new(polys: Vec<SectorPolygon>) -> SectorPolygons {
		SectorPolygons {
			polys,
			cur: 0
		}
	}
}

#[wasm_bindgen]
pub fn build_polygons(lines: Box<[usize]>, vertices: Box<[f32]>) -> Result<SectorPolygons, String> {
	if lines.len() & 1 != 0 {
		return Err(String::from("`lines` has an odd length!"));
	}
	let edges: Vec<Edge> = lines.chunks_exact(2)
		.map(|ch| Edge::new(ch[0], ch[1])).collect();
	if vertices.len() & 1 != 0 {
		return Err(String::from("`vertices` has an odd length!"));
	}
	let verts: Vec<Vector2> = vertices.chunks_exact(2)
		.map(|ch| Vector2::new(ch[0], ch[1])).collect();
	let polys = super::build_polygons(&edges, &verts);
	Ok(SectorPolygons::new(polys))
}

use crate::vector::Vector2;
use crate::edge::Edge;
use crate::vertex::MapVertex;
use std::collections::{HashMap, HashSet};

// Ported from https://github.com/pineapplemachine/jsdoom/blob/6dbc5540b8c7fd4a2c61dac9323fe0e77a51ddc6/src/convert/3DMapBuilder.ts#L117

/*
struct SectorPolygonBuilder<'a> {
	edges_used: HashMap<Edge, bool>,
	vertices: &'a Vec<MapVertex>
}
*/

fn angle_between(p1: Vector2, p2: Vector2, center: Vector2, clockwise: bool) -> f32 {
	let ab = p1 - &center;
	let cb = p2 - &center;
	let dot = ab.dot(&cb);
	let cross = ab.cross(&cb);
	f32::atan2(if clockwise {cross} else {-cross}, -dot)
}

pub fn build_polygons(lines: &Vec<Edge>, vertices: &Vec<MapVertex>) -> Vec<Vec<u32>> {
	let mut edges_used = HashMap::<Edge, bool>::new();
	let mut polygons: Vec<Vec<u32>> = Vec::new();
	polygons
}

fn find_next_start_edge(edges_used: &HashMap<Edge, bool>, vertices: &Vec<MapVertex>) -> Option<Vec<i32>> {
	let rightmost_vertex = edges_used.iter()
		.filter(|(&_key, &val)| val == false)
		.collect::<HashMap<&Edge, &bool>>()
		.keys()
		// Find usable vertices by destructuring the edges
		.fold(HashSet::<i32>::new(), |mut set, &edge| {
			edge.iter().for_each(|vertex_index| {
				set.insert(vertex_index);
			});
			set
		// Convert indices to vertices
		}).into_iter().map(|vertex_index| {
			&vertices[vertex_index as usize]
		}).max();
	if rightmost_vertex.is_none() { return None; }
	None
}

fn find_next_vertex(from: i32, previous: i32, clockwise: bool, lines: &Vec<Edge>, vertices: &Vec<MapVertex>) -> Option<i32> {
	None
}

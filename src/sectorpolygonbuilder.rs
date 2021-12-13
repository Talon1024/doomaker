use crate::vector::Vector2;
use crate::edge::Edge;
use crate::vertex::MapVertex;
use std::collections::{HashMap, HashSet};

// Ported from https://github.com/pineapplemachine/jsdoom/blob/6dbc5540b8c7fd4a2c61dac9323fe0e77a51ddc6/src/convert/3DMapBuilder.ts#L117

fn angle_between(p1: Vector2, p2: Vector2, center: Vector2, clockwise: bool) -> f32 {
	let ab = p1 - &center;
	let cb = p2 - &center;
	let dot = ab.dot(&cb);
	let cross = ab.cross(&cb);
	f32::atan2(if clockwise {cross} else {-cross}, -dot)
}

pub fn build_polygons(lines: &Vec<Edge>, vertices: &Vec<MapVertex>) -> Vec<Vec<i32>> {
	let mut edges_used = HashMap::<Edge, bool>::new();
	let mut polygons: Vec<Vec<i32>> = vec![
		find_next_start_edge(false, &edges_used, vertices).unwrap()];
	polygons
}

fn find_next_start_edge(clockwise: bool, edges_used: &HashMap<Edge, bool>, vertices: &Vec<MapVertex>) -> Option<Vec<i32>> {
	let usable_edges = edges_used.iter()
		.filter(|(&_key, &val)| val == false)
		.collect::<HashMap<&Edge, &bool>>();
	let rightmost_vertex = usable_edges.keys()
		// Find usable vertices by destructuring the edges
		.fold(HashSet::<i32>::new(), |mut set, &edge| {
			edge.iter().for_each(|vertex_index| {
				set.insert(vertex_index);
			});
			set
		// Convert indices to vertices
		}).into_iter().reduce(|current_index, other_index| {
			let current_vertex = vertices[current_index as usize];
			let other_vertex = vertices[other_index as usize];
			if other_vertex > current_vertex {
				other_index
			} else {
				current_index
			}
		});
	if rightmost_vertex.is_none() { return None; }
	let rightmost_vertex = rightmost_vertex.unwrap();
	let other_vertex = usable_edges.keys()
		.filter(|&key| key.contains(rightmost_vertex))
		.map(|&edge| edge.other(rightmost_vertex).unwrap())
		.reduce(|current_index, other_index| {
			// To ensure the interior angle is counterclockwise, pick the
			// connected vertex with the lowest angle. Necessary for proper
			// 3d-ification
			let rightmost_vertex = vertices[rightmost_vertex as usize].p;
			let current_vertex = vertices[current_index as usize].p;
			let other_vertex = vertices[other_index as usize].p;
			let current_angle = (rightmost_vertex - current_vertex).angle();
			let other_angle = (rightmost_vertex - other_vertex).angle();
			if !clockwise {
				if other_angle < current_angle {
					other_index
				} else {
					current_index
				}
			} else {
				if current_angle < other_angle {
					other_index
				} else {
					current_index
				}
			}
		});
	if other_vertex.is_none() { return None; }
	let other_vertex = other_vertex.unwrap();
	Some(vec![rightmost_vertex, other_vertex])
}

fn find_next_vertex(from: i32, previous: i32, clockwise: bool, edges: &HashMap<Edge, bool>, vertices: &Vec<MapVertex>) -> Option<i32> {
	None
}

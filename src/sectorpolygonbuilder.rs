#[forbid(unsafe_code)]
use crate::vector::Vector2;
use crate::edge::Edge;
use crate::vertex::MapVertex;
use std::collections::{HashMap, HashSet};

// Ported from https://github.com/pineapplemachine/jsdoom/blob/6dbc5540b8c7fd4a2c61dac9323fe0e77a51ddc6/src/convert/3DMapBuilder.ts#L117

fn rget<T: Copy>(vec: &Vec<T>, index: usize) -> T {
	let index = vec.len() - index;
	vec[index]
}

fn angle_between(
	p1: &Vector2,
	p2: &Vector2,
	center: &Vector2,
	clockwise: bool
) -> f32 {
	let ab = p1 - center;
	let cb = p2 - center;
	let dot = ab.dot(&cb);
	let cross = ab.cross(&cb);
	f32::atan2(if clockwise {cross} else {-cross}, -dot)
}

pub fn build_polygons(
	lines: &Vec<Edge>,
	vertices: &Vec<MapVertex>
) -> Vec<Vec<i32>> {
	// jsdoom's SectorPolygonBuilder takes care of duplicate vertices and
	// edges in its constructor. For this project, duplicate vertices and
	// edges should be taken care of when the level is being pre-processed.
	let mut edges_used = HashMap::<Edge, bool>::new();
	lines.iter().for_each(|&line| {
		edges_used.insert(line, false);
	});
	let first_edge = match find_next_start_edge(false, &edges_used, vertices) {
		Some(edge) => edge,
		None => return vec![]
	};
	let edge_count = edges_used.len();
	let mut polygons: Vec<Vec<i32>> = vec![first_edge];
	let mut clockwise = false;
	for _ in 0..edge_count {
		// polygons.last()[-2];
		let previous_vertex = rget(polygons.last().unwrap(), 2);
		// polygons.last()[-1];
		let current_vertex = rget(polygons.last().unwrap(), 1);
		let next_vertex = find_next_vertex(
			&current_vertex, &previous_vertex,
			&false, &edges_used, vertices
		);
		match next_vertex {
			Some(vertex) => {
				if is_polygon_complete(&polygons.last().unwrap(), vertex) {
					let first_edge = find_next_start_edge(false, &edges_used, vertices);
					match first_edge {
						Some(edge) => polygons.push(edge),
						None => break
					}
				} else {
					let edge = Edge::new(current_vertex, vertex);
					polygons.last_mut().unwrap().push(vertex);
					edges_used.insert(edge, true);
				}
			},
			None => {
				polygons.push(vec![]);
			}
		};
	}
	polygons
}

fn find_next_start_edge(
	clockwise: bool,
	edges: &HashMap<Edge, bool>,
	vertices: &Vec<MapVertex>
) -> Option<Vec<i32>> {
	// Filter out used edges
	let usable_edges = edges.iter()
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
		.map(|&edge| edge.other_unchecked(rightmost_vertex))
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

fn find_next_vertex(
	from: &i32,
	previous: &i32,
	clockwise: &bool,
	edges: &HashMap<Edge, bool>,
	vertices: &Vec<MapVertex>
) -> Option<i32> {
	let from = from.clone();
	let previous = previous.clone();
	let clockwise = clockwise.clone();
	// Find all edges that:
	// - Have not been added to a polygon
	// - Are attached to the "from" vertex
	// - Are not the "previous" vertex
	let usable_vertices = edges.keys()
		.filter(|&key| key.contains(from) && !key.contains(previous))
		.map(|&edge| edge.other_unchecked(from))
		.collect::<Vec<i32>>();
	if usable_vertices.len() == 0 { return None; }
	if usable_vertices.len() == 1 { return Some(usable_vertices[0]); }
	// Find the vertex with the lowest angle in comparison to "from"
	// The "previous" and "from" vertices will remain constant
	let previous_vertex = vertices[previous as usize];
	let from_vertex = vertices[from as usize];
	let next_vertex = usable_vertices.into_iter()
		.reduce(|current_index, other_index| {
			let current_vertex = vertices[current_index as usize];
			let other_vertex = vertices[other_index as usize];
			let current_angle = angle_between(
				&previous_vertex.p,
				&from_vertex.p,
				&current_vertex.p,
				clockwise
			);
			let other_angle = angle_between(
				&previous_vertex.p,
				&from_vertex.p,
				&other_vertex.p,
				clockwise
			);
			if other_angle < current_angle {
				other_index
			} else {
				current_index
			}
		}).unwrap();
	Some(next_vertex)
}

fn is_polygon_complete(polygon: &Vec<i32>, last: i32) -> bool {
	if polygon.len() < 3 {
		return false;
	}
	let first = polygon[0];
	return first == last;
}

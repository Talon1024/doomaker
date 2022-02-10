#[forbid(unsafe_code)]
use crate::vector::Vector2;
use crate::edge::Edge;
use crate::vertex::{self, MapVertex};
use std::collections::{HashMap, HashSet};
use ahash::RandomState;

// Ported from https://github.com/pineapplemachine/jsdoom/blob/6dbc5540b8c7fd4a2c61dac9323fe0e77a51ddc6/src/convert/3DMapBuilder.ts#L117

fn point_in_polygon(point: Vector2, polygon: &Vec<Vector2>) -> bool {
	// Based on https://wrf.ecse.rpi.edu/Research/Short_Notes/pnpoly.html
	let mut inside = false;
	for i in 0..polygon.len() {
		let j = if i == 0 { polygon.len() - 1 } else { i - 1 };
		let vi = polygon[i];
		let vj = polygon[j];
		if (
			(vi.y() > point.y()) != (vj.y() > point.y())) && (
			point.x() < (vj.x() - vi.x()) * (point.y() - vi.y()) / (vj.y() - vi.y()) + vi.x()
		) {
			inside = !inside;
		}
	}
	inside
}

fn edge_in_polygon(
	edge: &Edge,
	polygon: &Vec<i32>,
	map_vertices: &Vec<MapVertex>
) -> bool {
	let vertices = (
		map_vertices[edge.lo() as usize].p,
		map_vertices[edge.hi() as usize].p
	);
	let midpoint = vertex::midpoint(&vertices);
	let polygon: Vec<Vector2> = polygon.iter()
		.map(|&index| map_vertices[index as usize].p)
		.collect();
	point_in_polygon(midpoint, &polygon)
}

fn angle_between(
	p1: &Vector2,
	p2: &Vector2,
	center: &Vector2,
	clockwise: bool
) -> f32 {
	let ac = p1 - center;
	let bc = p2 - center;

	// Based on http://www.euclideanspace.com/maths/algebra/vectors/angleBetween/index.htm
	let ang = {
		(ac.angle() - bc.angle()) * (
			if clockwise {1.} else {-1.}
		)
	};

	if ang < 0.0 {
		ang + std::f32::consts::PI * 2.0
	} else if ang == -0.0 {
		panic!("Angle is -0.0");
		// std::f32::consts::PI
	} else {
		ang
	}
}

pub fn build_polygons(
	lines: &Vec<Edge>,
	vertices: &Vec<MapVertex>
) -> Vec<Vec<i32>> {
	// jsdoom's SectorPolygonBuilder takes care of duplicate vertices and
	// edges in its constructor. For this project, duplicate vertices and
	// edges should be taken care of when the level is being pre-processed.
	let mut edges_used: HashMap<Edge, bool, RandomState> = HashMap::default();
	lines.iter().for_each(|&line| {
		edges_used.insert(line, false);
	});
	let first_edge = match find_next_start_edge(false, &edges_used, vertices) {
		Some(edge) => edge,
		None => return vec![]
	};
	// let edge_count = edges_used.len();
	edges_used.insert(Edge::from(first_edge), true);
	let mut polygons: Vec<Vec<i32>> = vec![vec![first_edge.0, first_edge.1]];
	let mut incomplete_polygons: Vec<Vec<i32>> = Vec::new();
	let mut clockwise = false;
	loop {
		let mut poly_iter = polygons.last().unwrap().iter().copied().rev();
		// polygons.last()[-1];
		let current_vertex = poly_iter.next()
			.expect("A polygon should have at least one edge (two vertices)");
		// polygons.last()[-2];
		let previous_vertex = poly_iter.next()
			.expect("A polygon should have at least one edge (two vertices)");
		let next_vertex = find_next_vertex(
			&current_vertex, &previous_vertex,
			clockwise, &edges_used, vertices
		);
		#[cfg(test)]
		println!("pv {:<12} ls {:<12} nx {:<12}", previous_vertex, current_vertex, next_vertex.unwrap_or(-1));
		let mut new_polygon = false;
		match next_vertex {
			Some(vertex) => {
				let edge = Edge::new(current_vertex, vertex);
				edges_used.insert(edge, true);
				if is_polygon_complete(&polygons.last().unwrap(), vertex) {
					new_polygon = true;
				} else {
					polygons.last_mut().unwrap().push(vertex);
				}
			},
			None => {
				// The current polygon is probably incomplete
				let bad_polygon = polygons.pop().unwrap();
				incomplete_polygons.push(bad_polygon);
				new_polygon = true;
			}
		};
		if new_polygon {
			if let Some(edge) = find_next_start_edge(false, &edges_used, vertices) {
				polygons.push(vec![edge.0, edge.1]);
				let edge = Edge::from(edge);
				edges_used.insert(edge, true);
				polygons.iter().for_each(|polygon| {
					if edge_in_polygon(&edge, polygon, &vertices) {
						clockwise = !clockwise
					}
				});
			} else {
				break
			}
		}
	}
	polygons
}

fn find_next_start_edge(
	clockwise: bool,
	edges: &HashMap<Edge, bool, RandomState>,
	vertices: &Vec<MapVertex>
) -> Option<(i32, i32)> {
	// Filter out used edges
	let usable_edges: HashMap<&Edge, &bool> = edges.iter()
		.filter(|(&_key, &val)| val == false)
		.collect();
	let rightmost_vertex = usable_edges.keys()
		// Find usable vertices by destructuring the edges
		.fold(HashSet::<i32, RandomState>::default(), |mut set, &edge| {
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
		})?;
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
			if clockwise {
				if other_angle > current_angle {
					return other_index;
				}
			} else {
				if other_angle < current_angle {
					return other_index;
				}
			}
			current_index
		})?;
	Some((other_vertex, rightmost_vertex))
}

fn find_next_vertex(
	from: &i32,
	previous: &i32,
	clockwise: bool,
	edges: &HashMap<Edge, bool, RandomState>,
	vertices: &Vec<MapVertex>
) -> Option<i32> {
	let from = from.clone();
	let previous = previous.clone();
	// Find all edges that:
	// - Have not been added to a polygon
	// - Are attached to the "from" vertex
	// - Are not the "previous" vertex
	let usable_vertices: Vec<i32> = edges.iter()
		.filter_map(|(&key, &val)|
			if val == false && key.contains(from) && !key.contains(previous) {
				Some(key.other_unchecked(from))
			} else {
				None
			}
		).collect();
	if usable_vertices.len() == 0 { return None; }
	if usable_vertices.len() == 1 { return Some(usable_vertices[0]); }
	// Find the vertex with the lowest angle in comparison to "from"
	// The "previous" and "from" vertices will remain constant
	let previous_vertex = vertices[previous as usize];
	let from_vertex = vertices[from as usize];
	#[cfg(test)]
	println!("prev: {}, from: {}", previous, from);
	let next_vertex = usable_vertices.into_iter()
		.reduce(|current_index, other_index| {
			let current_vertex = vertices[current_index as usize];
			let other_vertex = vertices[other_index as usize];
			#[cfg(test)]
			println!("current: {}, other: {}", current_index, other_index);
			let current_angle = angle_between(
				&previous_vertex.p,
				&current_vertex.p,
				&from_vertex.p,
				clockwise
			);
			let other_angle = angle_between(
				&previous_vertex.p,
				&other_vertex.p,
				&from_vertex.p,
				clockwise
			);
			#[cfg(test)]
			println!("c angle: {}, o angle: {}", current_angle, other_angle);
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
	first == last
}

#[cfg(test)]
mod tests {
	use super::*;

	// see tests/data/simple.png for an annotated drawing of this data
	fn test_case_simple() -> (Vec<MapVertex>, HashMap<Edge, bool, RandomState>) {
		let verts: Vec<MapVertex> = vec![
			MapVertex { p: Vector2::from((0., 0.)) },
			MapVertex { p: Vector2::from((64., 0.)) },
			MapVertex { p: Vector2::from((64., -64.)) },
			MapVertex { p: Vector2::from((0., -64.)) },
			MapVertex { p: Vector2::from((0., 64.)) },
			MapVertex { p: Vector2::from((-64., 64.)) },
			MapVertex { p: Vector2::from((-64., 0.)) },
		];
		let lines: Vec<Edge> = vec![
			Edge::new(0, 1),
			Edge::new(1, 2),
			Edge::new(2, 3),
			Edge::new(3, 0),
			Edge::new(0, 4),
			Edge::new(4, 5),
			Edge::new(5, 6),
			Edge::new(6, 0),
		];
		let edges: HashMap<Edge, bool, RandomState> = lines.iter().map(
			|&line| (line, false)).collect();
		(verts, edges)
	}

	// ccw: counterclockwise, cw: clockwise

	#[test]
	fn correct_first_edge_ccw() {
		let (verts, edges) = test_case_simple();
		let first_edge = find_next_start_edge(false, &edges, &verts);
		assert_eq!(first_edge, Some((1, 2)));
	}

	#[test]
	fn correct_first_edge_cw() {
		let (verts, edges) = test_case_simple();
		let first_edge = find_next_start_edge(true, &edges, &verts);
		assert_eq!(first_edge, Some((3, 2)));
	}

	#[test]
	fn correct_next_vertex() {
		let (verts, edges) = test_case_simple();
		let from = 2;
		let previous = 3;

		let actual_vertex = find_next_vertex(&from, &previous, false, &edges, &verts);
		let expected_vertex = Some(1);
		assert_eq!(expected_vertex, actual_vertex);
	}

	#[test]
	fn correct_next_vertex_with_multiple_connected_edges_ccw() {
		let clockwise = false;
		let (verts, edges) = test_case_simple();

		// Inside lower right polygon
		let previous = 3;
		let from = 0;

		let actual_vertex = find_next_vertex(&from, &previous, clockwise, &edges, &verts);
		let expected_vertex = Some(1);
		assert_eq!(expected_vertex, actual_vertex);

		// Inside upper left polygon
		let previous = 4;
		let from = 0;

		let actual_vertex = find_next_vertex(&from, &previous, clockwise, &edges, &verts);
		let expected_vertex = Some(6);
		assert_eq!(expected_vertex, actual_vertex);

		// Outside on lower left
		let from = 0;
		let previous = 6;

		let actual_vertex = find_next_vertex(&from, &previous, clockwise, &edges, &verts);
		let expected_vertex = Some(3);
		assert_eq!(expected_vertex, actual_vertex);
	}

	#[test]
	fn correct_next_vertex_with_multiple_connected_edges_cw() {
		let clockwise = true;
		let (verts, edges) = test_case_simple();

		// Inside lower right polygon
		let from = 0;
		let previous = 1;

		let actual_vertex = find_next_vertex(&from, &previous, clockwise, &edges, &verts);
		let expected_vertex = Some(3);
		assert_eq!(expected_vertex, actual_vertex);

		// Inside upper left polygon
		let from = 0;
		let previous = 6;

		let actual_vertex = find_next_vertex(&from, &previous, clockwise, &edges, &verts);
		let expected_vertex = Some(4);
		assert_eq!(expected_vertex, actual_vertex);

		// Outside on lower left
		let from = 0;
		let previous = 3;

		let actual_vertex = find_next_vertex(&from, &previous, clockwise, &edges, &verts);
		let expected_vertex = Some(6);
		assert_eq!(expected_vertex, actual_vertex);
	}

	#[test]
	fn test_angle_between_ccw() {
		let clockwise = false;
		// p1
		// |  270 degrees
		// c -- p2
		let p1 = Vector2::from((0.0, 1.0));
		let p2 = Vector2::from((1.0, 0.0));
		let center = Vector2::from((0.0, 0.0));
		let angle = angle_between(&p1, &p2, &center, clockwise);
		assert_eq!(angle.to_degrees().round(), 270.0);

		// p2
		// |  90 degrees
		// c -- p1
		let p1 = Vector2::from((1.0, 0.0));
		let p2 = Vector2::from((0.0, 1.0));
		let center = Vector2::from((0.0, 0.0));
		let angle = angle_between(&p1, &p2, &center, clockwise);
		assert_eq!(angle.to_degrees().round(), 90.0);

		// c----p2
		//  \ 45 degrees
		//   p1
		let p1 = Vector2::from((1.0, -1.0));
		let p2 = Vector2::from((1.0, 0.0));
		let center = Vector2::from((0.0, 0.0));
		let angle = angle_between(&p1, &p2, &center, clockwise);
		assert_eq!(angle.to_degrees().round(), 45.0);

		//   c----p2
		//  / 135 degrees
		// p1
		let p1 = Vector2::from((-1.0, -1.0));
		let p2 = Vector2::from((1.0, 0.0));
		let center = Vector2::from((0.0, 0.0));
		let angle = angle_between(&p1, &p2, &center, clockwise);
		assert_eq!(angle.to_degrees().round(), 135.0);

		// p1
		//  \ 225 degrees
		//   c----p2
		let p1 = Vector2::from((-1.0, 1.0));
		let p2 = Vector2::from((1.0, 0.0));
		let center = Vector2::from((0.0, 0.0));
		let angle = angle_between(&p1, &p2, &center, clockwise);
		assert_eq!(angle.to_degrees().round(), 225.0);

		//   p1
		//  / 315 degrees
		// c----p2
		let p1 = Vector2::from((1.0, 1.0));
		let p2 = Vector2::from((1.0, 0.0));
		let center = Vector2::from((0.0, 0.0));
		let angle = angle_between(&p1, &p2, &center, clockwise);
		assert_eq!(angle.to_degrees().round(), 315.0);
	}

	#[test]
	fn test_angle_between_cw() {
		let clockwise = true;
		// p1
		// |  90 degrees
		// c -- p2
		let p1 = Vector2::from((0.0, 1.0));
		let p2 = Vector2::from((1.0, 0.0));
		let center = Vector2::from((0.0, 0.0));
		let angle = angle_between(&p1, &p2, &center, clockwise);
		assert_eq!(angle.to_degrees().round(), 90.0);

		// p2
		// |  270 degrees
		// c -- p1
		let p1 = Vector2::from((1.0, 0.0));
		let p2 = Vector2::from((0.0, 1.0));
		let center = Vector2::from((0.0, 0.0));
		let angle = angle_between(&p1, &p2, &center, clockwise);
		assert_eq!(angle.to_degrees().round(), 270.0);

		// c----p2
		//  \ 315 degrees
		//   p1
		let p1 = Vector2::from((1.0, -1.0));
		let p2 = Vector2::from((1.0, 0.0));
		let center = Vector2::from((0.0, 0.0));
		let angle = angle_between(&p1, &p2, &center, clockwise);
		assert_eq!(angle.to_degrees().round(), 315.0);

		//   c----p2
		//  / 225 degrees
		// p1
		let p1 = Vector2::from((-1.0, -1.0));
		let p2 = Vector2::from((1.0, 0.0));
		let center = Vector2::from((0.0, 0.0));
		let angle = angle_between(&p1, &p2, &center, clockwise);
		assert_eq!(angle.to_degrees().round(), 225.0);

		// p1
		//  \ 135 degrees
		//   c----p2
		let p1 = Vector2::from((-1.0, 1.0));
		let p2 = Vector2::from((1.0, 0.0));
		let center = Vector2::from((0.0, 0.0));
		let angle = angle_between(&p1, &p2, &center, clockwise);
		assert_eq!(angle.to_degrees().round(), 135.0);

		//   p1
		//  / 45 degrees
		// c----p2
		let p1 = Vector2::from((1.0, 1.0));
		let p2 = Vector2::from((1.0, 0.0));
		let center = Vector2::from((0.0, 0.0));
		let angle = angle_between(&p1, &p2, &center, clockwise);
		assert_eq!(angle.to_degrees().round(), 45.0);
	}

	#[test]
	fn correct_polygons() {
		let (verts, edges) = test_case_simple();
		let edges: Vec<Edge> = edges.keys().cloned().collect();
		let expected_polygons: Vec<Vec<i32>> = vec![vec![1, 2, 3, 0], vec![4, 0, 6, 5]];
		let actual_polygons = build_polygons(&edges, &verts);
		assert_eq!(expected_polygons, actual_polygons);
	}
}

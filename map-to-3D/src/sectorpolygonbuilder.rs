//! # Sector polygon builder
//! 
//! Takes a set of edges and vertices, and sorts them into "polygons"
//! consisting of vertex indices
use crate::vector::{Vector2, Coordinate};
use crate::edge::{Edge, EdgeVertexIndex};
use crate::vertex::MapVertex;
// use crate::boundingbox::BoundingBox;
use std::collections::{HashMap, HashSet};
use ahash::RandomState;

#[cfg(test)]
mod tests;

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
	polygon: &[EdgeVertexIndex],
	map_vertices: &[MapVertex]
) -> bool {
	let a = map_vertices[edge.lo() as usize].p;
	let b = map_vertices[edge.hi() as usize].p;
	let midpoint = a.midpoint(&b);
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
	#[cfg(micromath)]
	use micromath::F32;
	let ac = p1 - center;
	let bc = p2 - center;

	let ang = {
		cfg_if::cfg_if! {
			if #[cfg(atan_angle_calc)] {
				// Based on http://www.euclideanspace.com/maths/algebra/vectors/angleBetween/index.htm
				ac.angle() - bc.angle()
			} else {
				// Based on jsdoom: https://github.com/pineapplemachine/jsdoom/blob/04593d2c30311212ccb9af9aa503cf5c83465655/src/convert/3DMapBuilder.ts#L172
				// which was in itself based on what I learned after playing
				// around with 2D vector dot/cross products
				let d = ac.dot(&bc);
				let c = ac.cross(&bc);
				cfg_if::cfg_if! {
					if #[cfg(micromath)] {
						F32(c).atan2(F32(d)).0
					} else {
						c.atan2(d)
					}
				}
			}
		}
	} * if clockwise {-1.} else {1.};

	if ang < 0.0 {
		ang + std::f32::consts::PI * 2.0
	} else if ang == -0.0 {
		panic!("Angle is -0.0");
		// std::f32::consts::PI
	} else {
		ang
	}
}

/// A Sector Polygon
/// 
/// Consists of a list of vertices that make up the contour of the polygon, and
/// the index of the other polygon that this polygon is a hole of.
#[derive(PartialEq, Debug, Clone, Default)]
pub struct SectorPolygon {
	/// The indices of the vertices of this polygon's contour
	pub vertices: Vec<EdgeVertexIndex>,
	/// The index of the other polygon that this polygon is a hole of, if it is
	/// a hole in another polygon.
	pub hole_of: Option<usize>
}

/*
impl SectorPolygon {
	/// Get the bounding box for this polygon
	pub fn bounding_box(&self, vertices: &Vec<MapVertex>) -> BoundingBox {
		let vertices: Vec<MapVertex> = self.vertices.iter()
			.map(|&v| vertices[v as usize]).collect();
		let top = vertices.iter().map(|v| v.p.y()).reduce(f32::max).unwrap();
		let left = vertices.iter().map(|v| v.p.x()).reduce(f32::min).unwrap();
		let width = vertices.iter().map(|v| v.p.x()).reduce(f32::max).unwrap() - left;
		let height = top - vertices.iter().map(|v| v.p.y()).reduce(f32::min).unwrap();
		BoundingBox{top, left, width, height}
	}
}
*/

/// Build polygon contours from a set of lines and vertices.
/// 
/// Returns the polygon contours as a vector of vectors of contour vertex
/// indices, and which other polygons the polygons are holes of, as a vector of
/// optional indices of the first vector.
/// These can be used by a triangulator such as `earcut`.
/// 
/// # Examples
/// 
/// A square:
/// 
/// ```
/// use map_to_3D::vector::Vector2;
/// use map_to_3D::edge::Edge;
/// use map_to_3D::vertex::MapVertex;
/// use map_to_3D::sectorpolygonbuilder::{SectorPolygon, build_polygons};
/// 
/// // 3--0
/// // |  |
/// // 2--1
///
/// let vertices = vec![
/// 	MapVertex { p: Vector2::new(1.0, 1.0) },
/// 	MapVertex { p: Vector2::new(1.0, 0.0) },
/// 	MapVertex { p: Vector2::new(0.0, 0.0) },
/// 	MapVertex { p: Vector2::new(0.0, 1.0) },
/// ];
/// let lines = vec![
/// 	Edge::new(0, 1),
/// 	Edge::new(1, 2),
/// 	Edge::new(2, 3),
/// 	Edge::new(3, 0),
/// ];
/// assert_eq!(
/// 	build_polygons(&lines, &vertices),
/// 	// The polygon contour vertex index vector is nested because there can
/// 	// be multiple polygons, but this square is just one polygon. Also, the
/// 	// square is not a hole of another polygon.
/// 	vec![SectorPolygon { vertices: vec![0, 1, 2, 3], hole_of: None }]
/// )
/// ```
/// 
/// A simple example:
/// ```
/// use map_to_3D::vector::Vector2;
/// use map_to_3D::edge::Edge;
/// use map_to_3D::vertex::MapVertex;
/// use map_to_3D::sectorpolygonbuilder::{SectorPolygon, build_polygons};
/// 
/// // 5--4
/// // |  |
/// // 6--0--1
/// //    |  |
/// //    3--2
/// 
/// let verts: Vec<MapVertex> = vec![
/// 	MapVertex { p: Vector2::new(0., 0.) },
/// 	MapVertex { p: Vector2::new(64., 0.) },
/// 	MapVertex { p: Vector2::new(64., -64.) },
/// 	MapVertex { p: Vector2::new(0., -64.) },
/// 	MapVertex { p: Vector2::new(0., 64.) },
/// 	MapVertex { p: Vector2::new(-64., 64.) },
/// 	MapVertex { p: Vector2::new(-64., 0.) },
/// ];
/// let lines: Vec<Edge> = vec![
/// 	Edge::new(0, 1),
/// 	Edge::new(1, 2),
/// 	Edge::new(2, 3),
/// 	Edge::new(3, 0),
/// 	Edge::new(0, 4),
/// 	Edge::new(4, 5),
/// 	Edge::new(5, 6),
/// 	Edge::new(6, 0),
/// ];
/// 
/// let expected_polygons = vec![
/// 	SectorPolygon { vertices: vec![1, 2, 3, 0], hole_of: None },
/// 	SectorPolygon { vertices: vec![4, 0, 6, 5], hole_of: None }
/// ];
/// 
/// assert_eq!(
/// 	expected_polygons,
/// 	build_polygons(&lines, &verts)
/// );
/// 
/// ```
/// 
/// Example with a hole:
/// ```
/// use map_to_3D::vector::Vector2;
/// use map_to_3D::edge::Edge;
/// use map_to_3D::vertex::MapVertex;
/// use map_to_3D::sectorpolygonbuilder::{SectorPolygon, build_polygons};
/// 
/// // 0------1
/// // | 7--4 |
/// // | |  | |
/// // | 6--5 |
/// // 3------2
/// 
/// let verts: Vec<MapVertex> = vec![
/// 	MapVertex { p: Vector2::new(-7., 7.) }, // Outside
/// 	MapVertex { p: Vector2::new(7., 7.) },
/// 	MapVertex { p: Vector2::new(7., -7.) },
/// 	MapVertex { p: Vector2::new(-7., -7.) },
/// 	MapVertex { p: Vector2::new(5., 5.) }, // Hole
/// 	MapVertex { p: Vector2::new(5., -5.) },
/// 	MapVertex { p: Vector2::new(-5., -5.) },
/// 	MapVertex { p: Vector2::new(-5., 5.) },
/// ];
/// let lines: Vec<Edge> = vec![
/// 	Edge::new(0, 1),
/// 	Edge::new(1, 2),
/// 	Edge::new(2, 3),
/// 	Edge::new(3, 0),
/// 	Edge::new(4, 5),
/// 	Edge::new(5, 6),
/// 	Edge::new(6, 7),
/// 	Edge::new(7, 4),
/// ];
/// 
/// let expected_polygons = vec![
/// 	SectorPolygon { vertices: vec![1, 2, 3, 0], hole_of: None },
/// 	SectorPolygon { vertices: vec![4, 5, 6, 7], hole_of: Some(0) },
/// ];
/// 
/// assert_eq!(
/// 	expected_polygons,
/// 	build_polygons(&lines, &verts)
/// );
/// ```
/// 
/// Obviously, you won't hard-code the data into your program like this, but
/// it serves as an example of the kind of data you'll be passing into
/// `build_polygons`
/// 
/// # Panics
/// 
/// If there are two lines/edges overlapping each other, they could cause
/// angle_between to panic because the angle between them is -0
pub fn build_polygons(
	lines: &[Edge],
	vertices: &[MapVertex]
) -> Vec<SectorPolygon> {
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
	let mut polygons: Vec<SectorPolygon> = vec![SectorPolygon {
		vertices: vec![first_edge.0, first_edge.1],
		hole_of: None
	}];
	// Which polygons are holes, and which polygons are they holes of?
	let mut incomplete_polygons: Vec<SectorPolygon> = Vec::new();
	let mut clockwise = false;
	loop {
		let mut poly_iter = polygons.last().unwrap().vertices.iter().copied().rev();
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
		let mut new_polygon = false;
		match next_vertex {
			Some(vertex) => {
				let edge = Edge::new(current_vertex, vertex);
				edges_used.insert(edge, true);
				if is_polygon_complete(&polygons.last().unwrap().vertices, vertex) {
					new_polygon = true;
				} else {
					polygons.last_mut().unwrap().vertices.push(vertex);
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
				polygons.push(SectorPolygon{
					vertices: vec![edge.0, edge.1],
					hole_of: None
				});
				let edge = Edge::from(edge);
				edges_used.insert(edge, true);
				let mut inside_polygon_index: Option<usize> = None;
				clockwise = false;
				polygons.iter().enumerate().for_each(|(index, polygon)| {
					if edge_in_polygon(&edge, &polygon.vertices, &vertices) {
						clockwise = !clockwise;
						if clockwise {
							inside_polygon_index = Some(index);
						} else {
							inside_polygon_index = None;
						}
					}
				});
				polygons.last_mut().unwrap().hole_of = inside_polygon_index;
			} else {
				break
			}
		}
	}
	polygons
}

fn find_next_start_edge(
	clockwise: bool,  // Polygon's interior angles should be clockwise or not?
	edges: &HashMap<Edge, bool, RandomState>,
	vertices: &[MapVertex]
) -> Option<(EdgeVertexIndex, EdgeVertexIndex)> {
	// Filter out used edges
	let usable_edges: HashMap<&Edge, &bool> = edges.iter()
		.filter(|(&_key, &val)| val == false)
		.collect();
	let rightmost_vertex = usable_edges.keys()
		// Find usable vertices by destructuring the edges
		.fold(HashSet::<EdgeVertexIndex, RandomState>::default(), |mut set, &edge| {
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
	from: &EdgeVertexIndex,
	previous: &EdgeVertexIndex,
	clockwise: bool,
	edges: &HashMap<Edge, bool, RandomState>,
	vertices: &[MapVertex]
) -> Option<EdgeVertexIndex> {
	let from = from.clone();
	let previous = previous.clone();
	// Find all edges that:
	// - Have not been added to a polygon
	// - Are attached to the "from" vertex
	// - Are not the "previous" vertex
	let usable_vertices: Vec<EdgeVertexIndex> = edges.iter()
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
	let next_vertex = usable_vertices.into_iter()
		.reduce(|current_index, other_index| {
			let current_vertex = vertices[current_index as usize];
			let other_vertex = vertices[other_index as usize];
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
			if other_angle < current_angle {
				other_index
			} else {
				current_index
			}
		}).unwrap();
	Some(next_vertex)
}

fn is_polygon_complete(polygon: &Vec<EdgeVertexIndex>, last: EdgeVertexIndex) -> bool {
	if polygon.len() < 3 {
		return false;
	}
	let first = polygon[0];
	first == last
}

/// Convert the given polygon to a list of vertex indices for each triangle.
pub fn triangulate(
	polygon: &SectorPolygon,
	holes: &[&SectorPolygon],
	vertices: &[MapVertex]
) -> Vec<usize> {
	use std::iter;
	let vpos: Vec<Coordinate> = polygon.vertices.iter()
		.chain(holes.iter().flat_map(|h| h.vertices.iter()))
		.flat_map(|&i| (&vertices[i as usize]).xy()).collect();
	let mut cur_hole = polygon.vertices.len();
	let hole_indices: Vec<usize> = iter::once(cur_hole)
		.chain(holes.iter().map(|h| {
			let rv = h.vertices.len() + cur_hole;
			cur_hole += rv;
			rv
		})).take(holes.len()).collect();
	dbg!("{} vertices", vpos.len() / 2);
	dbg!("{}", &hole_indices);
	earcutr::earcut(&vpos, &hole_indices, 2)
}

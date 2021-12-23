use std::fmt;
// use std::str::FromStr;
// use std::num::ParseIntError;
use crate::vector::Vector2;

pub type EdgeVertexIndex = i32;

#[derive(Hash, PartialEq, Eq, Ord, PartialOrd, Clone, Debug, Copy)]
pub struct Edge(EdgeVertexIndex, EdgeVertexIndex);

impl fmt::Display for Edge {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.0, self.1)
	}
}

fn sort_edge(edge: Edge) -> Edge {
	// Sort in ascending order
	match edge {
		Edge(a, b) if b < a => Edge(b, a),
		_ => edge
	}
}

impl Edge {
	// Ensures deterministic order for edges
	pub fn new(a: EdgeVertexIndex, b: EdgeVertexIndex) -> Edge {
		sort_edge(Edge(a, b))
	}
	pub fn contains(&self, val: EdgeVertexIndex) -> bool {
		self.0 == val || self.1 == val
	}
	pub fn iter(&self) -> Iter {
		Iter {edge: &self, iter_index: 0}
	}
	pub fn other(&self, val: EdgeVertexIndex) -> Option<EdgeVertexIndex> {
		if self.0 == val {
			Some(self.1)
		} else if self.1 == val {
			Some(self.0)
		} else {
			None
		}
	}
	pub fn other_unchecked(&self, val: EdgeVertexIndex) -> EdgeVertexIndex {
		if self.0 == val {
			self.1
		} else {
			self.0
		}
	}
	pub fn midpoint(&self, vertices: &(Vector2, Vector2)) -> Vector2 {
		(vertices.0 + vertices.1) / 2.0
	}
	pub fn length(&self, vertices: &(Vector2, Vector2)) -> f32 {
		let relative_position = vertices.1 - vertices.0;
		relative_position.dot(&relative_position).sqrt()
	}
	pub fn start(&self) -> EdgeVertexIndex { self.0 }
	pub fn end(&self) -> EdgeVertexIndex { self.1 }
}

impl From<Edge> for Vec<EdgeVertexIndex> {
	fn from(edge: Edge) -> Vec<EdgeVertexIndex> {
		vec![edge.0, edge.1]
	}
}

impl From<&[EdgeVertexIndex]> for Edge {
	fn from(slice: &[EdgeVertexIndex]) -> Edge {
		Edge::new(
			slice.get(0).cloned().expect("No start vertex!"),
			slice.get(1).cloned().expect("No end vertex!")
		)
	}
}

/*
impl FromStr for Edge {
	type Err = ParseIntError;
	fn from_str(text: &str) -> Result<Self, Self::Err> {
		if let Some((a, b)) = text.split_once(" ") {
			let a = a.parse::<EdgeVertexIndex>()?;
			let b = b.parse::<EdgeVertexIndex>()?;
			Ok(Edge::new(a, b))
		} else {
			"a".parse::<EdgeVertexIndex>()
		}
	}
}
*/


pub struct Iter<'a> {
	edge: &'a Edge,
	iter_index: usize
}

impl<'a> Iterator for Iter<'a> {
	type Item = EdgeVertexIndex;
	fn next(&mut self) -> Option<Self::Item> {
		let result = match self.iter_index {
			0 => Some(self.edge.0),
			1 => Some(self.edge.1),
			_ => None
		};
		self.iter_index += 1;
		result
	}
}

impl<'a> ExactSizeIterator for Iter<'a> {
	fn len(&self) -> usize {
		2
	}
}

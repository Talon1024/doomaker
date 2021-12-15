use std::fmt;

#[derive(Hash, PartialEq, Eq, Ord, PartialOrd, Clone, Debug, Copy)]
pub struct Edge(i32, i32);

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
	pub fn new(a: i32, b: i32) -> Edge {
		sort_edge(Edge(a, b))
	}
	pub fn contains(&self, val: i32) -> bool {
		self.0 == val || self.1 == val
	}
	pub fn iter(&self) -> Iter {
		Iter {edge: &self, iter_index: 0}
	}
	pub fn other(&self, val: i32) -> Option<i32> {
		if self.0 == val {
			Some(self.1)
		} else if self.1 == val {
			Some(self.0)
		} else {
			None
		}
	}
	pub fn other_unchecked(&self, val: i32) -> i32 {
		if self.0 == val {
			self.1
		} else {
			self.0
		}
	}
}

impl From<Edge> for Vec<i32> {
	fn from(edge: Edge) -> Vec<i32> {
		vec![edge.0, edge.1]
	}
}

impl From<&[i32]> for Edge {
	fn from(slice: &[i32]) -> Edge {
		Edge::new(
			slice.get(0).cloned().expect("No start vertex!"),
			slice.get(1).cloned().expect("No end vertex!")
		)
	}
}

pub struct Iter<'a> {
	edge: &'a Edge,
	iter_index: usize
}

impl<'a> Iterator for Iter<'a> {
	type Item = i32;
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

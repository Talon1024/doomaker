//! # Edges
//! 
//! An edge connects two vertices. The `Edge` struct contains two vertex
//! indices, and it can be used as a hash map key or hash set item.
//! 
//! The `Edge` struct underpins this entire crate.

use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;

pub type EdgeVertexIndex = i32;

/// An "Edge" - a connection between two points
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
	/// Create a new `Edge`. This function also ensures that the vertex
	/// indices are always sorted in ascending order.
	/// 
	/// # Panics
	/// 
	/// Panics if `a == b`
	/// 
	/// An edge with two vertex indices which are the same is invalid,
	/// and such bad edges are expected to be filtered out by the program
	/// before attempting to create new `Edge`s. It's also partly laziness
	/// on my part, since changing 100+ calls to this function would quickly
	/// get tedious.
	/// 
	/// # Example
	/// 
	/// ```
	/// use map_to_3D::edge::Edge;
	/// let edge = Edge::new(4, 1);
	/// // Note the numbers are the same, even though they are in a different
	/// // order. They are sorted in ascending order to prevent duplication.
	/// assert_eq!(edge, Edge::new(1, 4));
	/// ```
	pub fn new(a: EdgeVertexIndex, b: EdgeVertexIndex) -> Edge {
		if a == b {
			panic!("The edge should use two different vertices");
		}
		// Ensures deterministic order for edges
		sort_edge(Edge(a, b))
	}
	/// Does this edge use the given vertex index?
	/// 
	/// # Examples
	/// 
	/// ```
	/// use map_to_3D::edge::Edge;
	/// let edge = Edge::new(4, 1);
	/// assert!(edge.contains(4));
	/// assert!(edge.contains(1));
	/// ```
	/// 
	/// ```
	/// use map_to_3D::edge::Edge;
	/// let edge = Edge::new(4, 1);
	/// assert!(!edge.contains(2));
	/// ```
	pub fn contains(&self, val: EdgeVertexIndex) -> bool {
		self.0 == val || self.1 == val
	}
	/// Create an iterator for this edge
	pub fn iter(&self) -> Iter {
		Iter {edge: &self, iter_index: 0}
	}
	/// Get the other vertex index for this edge, if the given vertex index
	/// matches one of the vertex indices in this edge.
	/// 
	/// # Examples
	/// 
	/// ```
	/// use map_to_3D::edge::Edge;
	/// let edge = Edge::new(4, 1);
	/// assert_eq!(edge.other(4), Some(1));
	/// ```
	/// 
	/// ```
	/// use map_to_3D::edge::Edge;
	/// let edge = Edge::new(4, 1);
	/// assert_eq!(edge.other(2), None);
	/// ```
	pub fn other(&self, index: EdgeVertexIndex) -> Option<EdgeVertexIndex> {
		if self.0 == index {
			Some(self.1)
		} else if self.1 == index {
			Some(self.0)
		} else {
			None
		}
	}
	/// Get the other vertex index for this edge, if the given vertex index
	/// matches one of the vertex indices in this edge.
	/// 
	/// Unlike `other()`, this does not ensure the given vertex index is
	/// actually one of the vertex indices this edge uses. If the given vertex
	/// index does not match the first/low vertex index, the low vertex index
	/// is returned. Otherwise, the high vertex index is returned.
	/// 
	/// # Examples
	/// 
	/// ```
	/// use map_to_3D::edge::Edge;
	/// let edge = Edge::new(4, 1);
	/// assert_eq!(edge.other_unchecked(4), 1);
	/// ```
	/// 
	/// ```
	/// use map_to_3D::edge::Edge;
	/// let edge = Edge::new(4, 1);
	/// assert_eq!(edge.other_unchecked(2), 1);
	/// ```
	pub fn other_unchecked(&self, index: EdgeVertexIndex) -> EdgeVertexIndex {
		if self.0 == index {
			self.1
		} else {
			self.0
		}
	}
	/// Get the low vertex index for this edge
	/// 
	/// # Example
	/// 
	/// ```
	/// use map_to_3D::edge::Edge;
	/// let edge = Edge::new(4, 1);
	/// assert_eq!(edge.lo(), 1);
	/// ```
	pub fn lo(&self) -> EdgeVertexIndex { self.0 }
	/// Get the high vertex index for this edge
	/// 
	/// # Example
	/// 
	/// ```
	/// use map_to_3D::edge::Edge;
	/// let edge = Edge::new(4, 1);
	/// assert_eq!(edge.hi(), 4);
	/// ```
	pub fn hi(&self) -> EdgeVertexIndex { self.1 }
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

impl From<(EdgeVertexIndex, EdgeVertexIndex)> for Edge {
	fn from(v: (EdgeVertexIndex, EdgeVertexIndex)) -> Edge {
		Edge::new(v.0, v.1)
	}
}

pub enum EdgeFromStrError {
	ParseError(ParseIntError),
	ValueError
}

impl From<ParseIntError> for EdgeFromStrError {
	fn from(err: ParseIntError) -> EdgeFromStrError {
		EdgeFromStrError::ParseError(err)
	}
}

impl fmt::Display for EdgeFromStrError {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match self {
			EdgeFromStrError::ParseError(err) => {
				err.fmt(f)
			},
			EdgeFromStrError::ValueError => {
				f.write_fmt(format_args!("{}", "Insufficient values available"))
			}
		}
	}
}

impl FromStr for Edge {
	type Err = EdgeFromStrError;
	fn from_str(text: &str) -> Result<Self, Self::Err> {

		use EdgeFromStrError::ValueError;

		let mut nums_iter = text.split_ascii_whitespace();
		let a = nums_iter.next().ok_or(ValueError)?;
		let b = nums_iter.next().ok_or(ValueError)?;

		let a = a.parse::<EdgeVertexIndex>()?;
		let b = b.parse::<EdgeVertexIndex>()?;
		Ok(Edge::new(a, b))
	}
}

/// Iterator for edges. Yields the vertex indices in ascending order.
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

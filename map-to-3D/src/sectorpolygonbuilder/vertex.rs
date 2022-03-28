//! # Map vertex
//! 
//! This is mostly a thin wrapper around Vector2, so that it can be sorted
//! differently.
use std::cmp::Ordering::{self, Equal, Greater, Less};
use crate::vector::Vector2;

#[derive(PartialEq, Clone, Debug, Copy)]
pub(super) struct MapVertex<'a> {
	/// The point
	pub p: &'a Vector2,
	/// The map vertex index
	pub i: usize,
}

impl<'a> Eq for MapVertex<'a>{}
impl<'a> Ord for MapVertex<'a> {
	fn cmp(&self, other: &Self) -> Ordering {
		/*
		For reference:
		assert_eq!(5.cmp(&10), Ordering::Less);
		assert_eq!(10.cmp(&5), Ordering::Greater);
		assert_eq!(5.cmp(&5), Ordering::Equal);
		*/
		let sx = self.p.x();
		let sy = self.p.y();
		let ox = other.p.x();
		let oy = other.p.y();
		if ox == sx {
			if oy < sy {
				Less
			} else if oy == sy {
				Equal
			} else {
				Greater
			}
		} else if sx > ox {
			Greater
		} else {
			Less
		}
	}
}

impl<'a> PartialOrd for MapVertex<'a> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		// Should not panic because it never returns None
		Some(self.cmp(other))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	// see tests/data/simple.png for an annotated drawing of this data
	fn test_case_simple() -> Vec<Vector2> {
		vec![
			Vector2::new(0., 0.),
			Vector2::new(64., 0.),
			Vector2::new(64., -64.),
			Vector2::new(0., -64.),
			Vector2::new(0., 64.),
			Vector2::new(-64., 64.),
			Vector2::new(-64., 0.),
		]
	}

	#[test]
	fn correct_max_vertex() {
		let verts = test_case_simple();
		let mverts: Vec<MapVertex> = verts.iter().enumerate()
			.map(|(i, v)| MapVertex {p: &v, i}).collect();
		let expected = &mverts[2];
		let actual = mverts.iter().max().unwrap();
		assert_eq!(expected, actual);
	}

	#[test]
	fn correct_min_vertex() {
		let verts = test_case_simple();
		let mverts: Vec<MapVertex> = verts.iter().enumerate()
			.map(|(i, v)| MapVertex {p: &v, i}).collect();
		let expected = &mverts[5];
		let actual = mverts.iter().min().unwrap();
		assert_eq!(expected, actual);
	}

	#[test]
	fn correct_lt_comparison() {
		let verts = test_case_simple();
		let mverts: Vec<MapVertex> = verts.iter().enumerate()
			.map(|(i, v)| MapVertex {p: &v, i}).collect();
		assert_eq!(mverts[3].cmp(&mverts[2]), Less);
		assert_eq!(mverts[1].cmp(&mverts[2]), Less);
		assert_eq!(mverts[3] < mverts[2], true);
		assert_eq!(mverts[1] < mverts[2], true);
	}

	#[test]
	fn correct_gt_comparison() {
		let verts = test_case_simple();
		let mverts: Vec<MapVertex> = verts.iter().enumerate()
			.map(|(i, v)| MapVertex {p: &v, i}).collect();
		assert_eq!(mverts[0].cmp(&mverts[6]), Greater);
		assert_eq!(mverts[0].cmp(&mverts[4]), Greater);
		assert_eq!(mverts[0] > mverts[6], true);
		assert_eq!(mverts[0] > mverts[4], true);
	}
}

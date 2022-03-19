//! # Map vertex
//! 
//! This is mostly a thin wrapper around Vector2, so that it can be sorted
//! differently.
use std::cmp::Ordering::{self, Equal, Greater, Less};
use crate::vector::{Vector2, Coordinate, Iter as VIter};

#[derive(PartialEq, Clone, Debug, Copy, Default)]
pub struct MapVertex {
	pub p: Vector2
}

impl MapVertex {
	pub fn xy(&self) -> VIter {
		self.p.xy()
	}
}

impl From<Vector2> for MapVertex {
	fn from(v: Vector2) -> MapVertex {
		MapVertex { p: v }
	}
}

impl From<&[Coordinate]> for MapVertex {
	fn from(v: &[Coordinate]) -> MapVertex {
		MapVertex { p: Vector2::from(v) }
	}
}

impl Eq for MapVertex{}
impl Ord for MapVertex {
	fn cmp(&self, other: &Self) -> Ordering {
		/*
		For reference:
		assert_eq!(5.cmp(&10), Ordering::Less);
		assert_eq!(10.cmp(&5), Ordering::Greater);
		assert_eq!(5.cmp(&5), Ordering::Equal);
		*/
		if other.p.x() == self.p.x() {
			if other.p.y() < self.p.y() {
				Less
			} else if other.p.y() == self.p.y() {
				Equal
			} else {
				Greater
			}
		} else if self.p.x() > other.p.x() {
			Greater
		} else {
			Less
		}
	}
}

impl PartialOrd for MapVertex {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		// Should not panic because it never returns None
		Some(self.cmp(other))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	// see tests/data/simple.png for an annotated drawing of this data
	fn test_case_simple() -> Vec<MapVertex> {
		let verts: Vec<MapVertex> = vec![
			MapVertex { p: Vector2::new(0., 0.) },
			MapVertex { p: Vector2::new(64., 0.) },
			MapVertex { p: Vector2::new(64., -64.) },
			MapVertex { p: Vector2::new(0., -64.) },
			MapVertex { p: Vector2::new(0., 64.) },
			MapVertex { p: Vector2::new(-64., 64.) },
			MapVertex { p: Vector2::new(-64., 0.) },
		];
		verts
	}

	#[test]
	fn correct_max_vertex() {
		let verts = test_case_simple();
		assert_eq!(
			MapVertex { p: Vector2::new(64., -64.) },
			verts.iter().max().cloned().unwrap());
	}

	#[test]
	fn correct_min_vertex() {
		let verts = test_case_simple();
		assert_eq!(
			MapVertex { p: Vector2::new(-64., 64.) },
			verts.iter().min().cloned().unwrap());
	}

	#[test]
	fn correct_lt_comparison() {
		let verts = test_case_simple();
		assert_eq!(verts[3].cmp(&verts[2]), Less);
		assert_eq!(verts[1].cmp(&verts[2]), Less);
		assert_eq!(verts[3] < verts[2], true);
		assert_eq!(verts[1] < verts[2], true);
	}

	#[test]
	fn correct_gt_comparison() {
		let verts = test_case_simple();
		assert_eq!(verts[0].cmp(&verts[6]), Greater);
		assert_eq!(verts[0].cmp(&verts[4]), Greater);
		assert_eq!(verts[0] > verts[6], true);
		assert_eq!(verts[0] > verts[4], true);
	}
}

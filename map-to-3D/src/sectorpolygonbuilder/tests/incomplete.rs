use super::*;
use data::test_case_incomplete;

#[test]
fn correct_polygons() {
	let (verts, edges) = test_case_incomplete();
	let expected_polygons: Vec<Vec<i32>> = vec![vec![4, 5, 6, 3]];
	let expected_holes: Vec<Option<usize>> = vec![None];
	let (actual_polygons, holes) = build_polygons(&edges, &verts);
	assert_eq!(expected_polygons, actual_polygons);
	assert_eq!(expected_holes, holes);
}

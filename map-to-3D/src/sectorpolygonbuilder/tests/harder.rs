use super::*;
use data::test_case_harder;

#[test]
fn correct_polygons() {
	let (verts, edges) = test_case_harder();
	let expected_polygons: Vec<Vec<i32>> = vec![
		vec![1, 0, 2, 9],
		vec![10, 2, 7, 5],
		vec![8, 9, 10, 11],
		vec![6, 1, 8, 3],
		vec![11, 5, 4, 3],
	];
	let expected_holes: Vec<Option<usize>> = vec![None, None, None, None, None];
	let (actual_polygons, holes) = build_polygons(&edges, &verts);
	assert_eq!(expected_polygons, actual_polygons);
	assert_eq!(holes, expected_holes);
}

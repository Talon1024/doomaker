use super::*;
use data::test_case_incomplete;

#[test]
fn correct_polygons() {
	let (verts, edges) = test_case_incomplete();
	let expected_polygons: Vec<SectorPolygon> = vec![
		SectorPolygon { vertices: vec![4, 5, 6, 3], hole_of: None }
	];
	let actual_polygons = build_polygons(&edges, &verts);
	assert_eq!(expected_polygons, actual_polygons);
}

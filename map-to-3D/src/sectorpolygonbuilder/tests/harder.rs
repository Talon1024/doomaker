use super::*;
use data::test_case_harder;

#[test]
fn correct_polygons() {
	let (verts, edges) = test_case_harder();
	let expected_polygons: Vec<SectorPolygon> = vec![
		SectorPolygon { vertices: vec![1, 0, 2, 9], hole_of: None },
		SectorPolygon { vertices: vec![10, 2, 7, 5], hole_of: None },
		SectorPolygon { vertices: vec![8, 9, 10, 11], hole_of: None },
		SectorPolygon { vertices: vec![6, 1, 8, 3], hole_of: None },
		SectorPolygon { vertices: vec![11, 5, 4, 3], hole_of: None },
	];
	let actual_polygons = build_polygons(&edges, &verts);
	assert_eq!(expected_polygons, actual_polygons);
}

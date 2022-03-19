use super::*;
use data::test_case_insides;

#[test]
fn correct_polygons() {
	let (verts, edges) = test_case_insides();
	let expected_polygons: Vec<SectorPolygon> = vec![
		SectorPolygon { vertices: vec![8, 9, 10, 11], hole_of: None },
		SectorPolygon { vertices: vec![1, 2, 3, 4, 5, 6, 7, 0], hole_of: Some(0) },
		SectorPolygon { vertices: vec![7, 1, 3, 5], hole_of: None },
	];
	let actual_polygons = build_polygons(&edges, &verts);
	assert_eq!(expected_polygons, actual_polygons);

	let polygon_index = 0usize;
	let the_polygon = &actual_polygons[polygon_index];
	let holes: Vec<&SectorPolygon> = actual_polygons
		.iter().filter(|p| p.hole_of == Some(polygon_index))
		.collect();
	let triangles = triangulate(the_polygon, &holes, &verts);

	// 3 triangles for each side, 3 vertices for each triangle, 4 sides
	assert_eq!(triangles.len(), 3 * 3 * 4);
}

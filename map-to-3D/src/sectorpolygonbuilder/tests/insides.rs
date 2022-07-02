use super::*;

// see tests/data/insides.png for an annotated drawing of this data
fn test_case_insides() -> (Vec<Vec2>, Vec<Edge>) {
	let verts: Vec<Vec2> = vec![
		Vec2::new(64., 64.),     // 0
		Vec2::new(64., 0.),
		Vec2::new(64., -64.),
		Vec2::new(0., -64.),
		Vec2::new(-64., -64.),       // 4
		Vec2::new(-64., 0.),
		Vec2::new(-64., 64.),
		Vec2::new(0., 64.),
		Vec2::new(99., 99.),       // 8
		Vec2::new(99., -99.),
		Vec2::new(-99., -99.),
		Vec2::new(-99., 99.),       // 11
	];
	let edges: Vec<Edge> = vec![
		Edge::new(8, 9),
		Edge::new(10, 9),
		Edge::new(11, 10),
		Edge::new(11, 8),   // End of outer square
		Edge::new(0, 1),
		Edge::new(7, 0),
		Edge::new(2, 1),
		Edge::new(2, 3),
		Edge::new(3, 4),
		Edge::new(4, 5),
		Edge::new(5, 6),
		Edge::new(6, 7),
		Edge::new(7, 0),    // End of inner square
		Edge::new(7, 1),
		Edge::new(1, 3),
		Edge::new(3, 5),
		Edge::new(5, 7),    // End of inner diamond
	];
	(verts, edges)
}

#[test]
fn correct_polygons() {
	let (verts, edges) = test_case_insides();
	let expected_polygons: Vec<SectorPolygon> = vec![
		SectorPolygon { vertices: vec![8, 9, 10, 11], hole_of: None },
		SectorPolygon { vertices: vec![1, 2, 3, 4, 5, 6, 7, 0], hole_of: Some(0) },
		SectorPolygon { vertices: vec![3, 1, 7, 5], hole_of: None },
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

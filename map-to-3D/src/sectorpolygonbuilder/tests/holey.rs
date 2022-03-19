use super::*;

// See tests/data/holey.png for an illustration
fn test_case() -> (Vec<MapVertex>, Vec<Edge>) {
	let verts: Vec<MapVertex> = vec![
		MapVertex { p: Vector2::new(64., 64.) },	// 0
		MapVertex { p: Vector2::new(64., -64.) },
		MapVertex { p: Vector2::new(-64., -64.) },
		MapVertex { p: Vector2::new(-64., 64.) },
		MapVertex { p: Vector2::new(44., 52.) },	// 4
		MapVertex { p: Vector2::new(-52., 52.) },
		MapVertex { p: Vector2::new(-52., -44.) },
		MapVertex { p: Vector2::new(52., 44.) },
		MapVertex { p: Vector2::new(52., -52.) },	// 8
		MapVertex { p: Vector2::new(-44., -52.) },
	];
	let edges = vec![
		Edge::new(0, 1),
		Edge::new(1, 2),
		Edge::new(2, 3),
		Edge::new(3, 0),
		Edge::new(4, 5),
		Edge::new(5, 6),
		Edge::new(6, 4),
		Edge::new(7, 8),
		Edge::new(8, 9),
		Edge::new(9, 7),
	];
	(verts, edges)
}

#[test]
fn holey() {
	let (verts, edges) = test_case();
	let polys = build_polygons(&edges, &verts);
	assert_eq!(polys[1].hole_of, Some(0));
	assert_eq!(polys[2].hole_of, Some(0));

	let polygon_index = 0usize;
	let the_polygon = &polys[polygon_index];
	let holes: Vec<&SectorPolygon> = polys
		.iter().filter(|p| p.hole_of == Some(polygon_index)).collect();
	let triangles = triangulate(the_polygon, &holes, &verts);

	assert_eq!(triangles.len(), 12 * 3);
}

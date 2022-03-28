use super::*;

// see tests/data/simple.png for an annotated drawing of this data
fn test_case_simple() -> (Vec<Vector2>, Vec<Edge>) {
	let verts: Vec<Vector2> = vec![
		Vector2::new(0., 0.),
		Vector2::new(64., 0.),
		Vector2::new(64., -64.),
		Vector2::new(0., -64.),
		Vector2::new(0., 64.),
		Vector2::new(-64., 64.),
		Vector2::new(-64., 0.),
	];
	let edges: Vec<Edge> = vec![
		Edge::new(0, 1),
		Edge::new(1, 2),
		Edge::new(2, 3),
		Edge::new(3, 0),
		Edge::new(0, 4),
		Edge::new(4, 5),
		Edge::new(5, 6),
		Edge::new(6, 0),
	];
	(verts, edges)
}

#[test]
fn correct_first_edge_ccw() {
	let (verts, edges) = test_case_simple();
	let edges: HashMap<Edge, bool, RandomState> = 
		edges.into_iter().map(|e| (e, false)).collect();
	let verts: Vec<MapVertex> = verts.iter().enumerate()
		.map(|(i, v)| MapVertex { p: v.clone(), i }).collect();
	let first_edge = find_next_start_edge(false, &edges, &verts);
	assert_eq!(first_edge, Some((1, 2)));
}

#[test]
fn correct_first_edge_cw() {
	let (verts, edges) = test_case_simple();
	let edges: HashMap<Edge, bool, RandomState> = 
		edges.into_iter().map(|e| (e, false)).collect();
	let verts: Vec<MapVertex> = verts.iter().enumerate()
		.map(|(i, v)| MapVertex { p: v.clone(), i }).collect();
	let first_edge = find_next_start_edge(true, &edges, &verts);
	assert_eq!(first_edge, Some((3, 2)));
}

#[test]
fn correct_next_vertex() {
	let (verts, edges) = test_case_simple();
	let edges: HashMap<Edge, bool, RandomState> = 
		edges.into_iter().map(|e| (e, false)).collect();
	let verts: Vec<MapVertex> = verts.iter().enumerate()
		.map(|(i, v)| MapVertex { p: v.clone(), i }).collect();
	let from = 2;
	let previous = 3;

	let actual_vertex = find_next_vertex(&from, &previous, false, &edges, &verts);
	let expected_vertex = Some(1);
	assert_eq!(expected_vertex, actual_vertex);
}

#[test]
fn correct_next_vertex_with_multiple_connected_edges_ccw() {
	let clockwise = false;
	let (verts, edges) = test_case_simple();
	let verts: Vec<MapVertex> = verts.iter().enumerate()
		.map(|(i, v)| MapVertex { p: v.clone(), i }).collect();
	let edges: HashMap<Edge, bool, RandomState> = 
		edges.into_iter().map(|e| (e, false)).collect();

	// Inside lower right polygon
	let previous = 3;
	let from = 0;

	let actual_vertex = find_next_vertex(&from, &previous, clockwise, &edges, &verts);
	let expected_vertex = Some(1);
	assert_eq!(expected_vertex, actual_vertex);

	// Inside upper left polygon
	let previous = 4;
	let from = 0;

	let actual_vertex = find_next_vertex(&from, &previous, clockwise, &edges, &verts);
	let expected_vertex = Some(6);
	assert_eq!(expected_vertex, actual_vertex);

	// Outside on lower left
	let from = 0;
	let previous = 6;

	let actual_vertex = find_next_vertex(&from, &previous, clockwise, &edges, &verts);
	let expected_vertex = Some(3);
	assert_eq!(expected_vertex, actual_vertex);
}

#[test]
fn correct_next_vertex_with_multiple_connected_edges_cw() {
	let clockwise = true;
	let (verts, edges) = test_case_simple();
	let edges: HashMap<Edge, bool, RandomState> = 
		edges.into_iter().map(|e| (e, false)).collect();
	let verts: Vec<MapVertex> = verts.iter().enumerate()
		.map(|(i, v)| MapVertex { p: v.clone(), i }).collect();

	// Inside lower right polygon
	let from = 0;
	let previous = 1;

	let actual_vertex = find_next_vertex(&from, &previous, clockwise, &edges, &verts);
	let expected_vertex = Some(3);
	assert_eq!(expected_vertex, actual_vertex);

	// Inside upper left polygon
	let from = 0;
	let previous = 6;

	let actual_vertex = find_next_vertex(&from, &previous, clockwise, &edges, &verts);
	let expected_vertex = Some(4);
	assert_eq!(expected_vertex, actual_vertex);

	// Outside on lower left
	let from = 0;
	let previous = 3;

	let actual_vertex = find_next_vertex(&from, &previous, clockwise, &edges, &verts);
	let expected_vertex = Some(6);
	assert_eq!(expected_vertex, actual_vertex);
}

#[test]
fn correct_polygons() {
	let (verts, edges) = test_case_simple();
	let expected_polygons: Vec<SectorPolygon> = vec![
		SectorPolygon { vertices: vec![1, 2, 3, 0], hole_of: None },
		SectorPolygon { vertices: vec![4, 0, 6, 5], hole_of: None },
	];
	let actual_polygons = build_polygons(&edges, &verts);
	assert_eq!(expected_polygons, actual_polygons);
}

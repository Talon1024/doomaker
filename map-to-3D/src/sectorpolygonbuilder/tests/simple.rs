use super::*;
use data::test_case_simple;

#[test]
fn correct_first_edge_ccw() {
	let (verts, edges) = test_case_simple();
	let edges: HashMap<Edge, bool, RandomState> = 
		edges.into_iter().map(|e| (e, false)).collect();
	let first_edge = find_next_start_edge(false, &edges, &verts);
	assert_eq!(first_edge, Some((1, 2)));
}

#[test]
fn correct_first_edge_cw() {
	let (verts, edges) = test_case_simple();
	let edges: HashMap<Edge, bool, RandomState> = 
		edges.into_iter().map(|e| (e, false)).collect();
	let first_edge = find_next_start_edge(true, &edges, &verts);
	assert_eq!(first_edge, Some((3, 2)));
}

#[test]
fn correct_next_vertex() {
	let (verts, edges) = test_case_simple();
	let edges: HashMap<Edge, bool, RandomState> = 
		edges.into_iter().map(|e| (e, false)).collect();
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

use super::*;

// see tests/data/simple.png for an annotated drawing of this data
fn test_case_simple() -> (Vec<MapVertex>, HashMap<Edge, bool, RandomState>) {
    let verts: Vec<MapVertex> = vec![
        MapVertex { p: Vector2::from((0., 0.)) },
        MapVertex { p: Vector2::from((64., 0.)) },
        MapVertex { p: Vector2::from((64., -64.)) },
        MapVertex { p: Vector2::from((0., -64.)) },
        MapVertex { p: Vector2::from((0., 64.)) },
        MapVertex { p: Vector2::from((-64., 64.)) },
        MapVertex { p: Vector2::from((-64., 0.)) },
    ];
    let lines: Vec<Edge> = vec![
        Edge::new(0, 1),
        Edge::new(1, 2),
        Edge::new(2, 3),
        Edge::new(3, 0),
        Edge::new(0, 4),
        Edge::new(4, 5),
        Edge::new(5, 6),
        Edge::new(6, 0),
    ];
    let edges: HashMap<Edge, bool, RandomState> = lines.iter().map(
        |&line| (line, false)).collect();
    (verts, edges)
}

// ccw: counterclockwise, cw: clockwise

#[test]
fn correct_first_edge_ccw() {
    let (verts, edges) = test_case_simple();
    let first_edge = find_next_start_edge(false, &edges, &verts);
    assert_eq!(first_edge, Some((1, 2)));
}

#[test]
fn correct_first_edge_cw() {
    let (verts, edges) = test_case_simple();
    let first_edge = find_next_start_edge(true, &edges, &verts);
    assert_eq!(first_edge, Some((3, 2)));
}

#[test]
fn correct_next_vertex() {
    let (verts, edges) = test_case_simple();
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
fn test_angle_between_ccw() {
    let clockwise = false;
    // p1
    // |  270 degrees
    // c -- p2
    let p1 = Vector2::from((0.0, 1.0));
    let p2 = Vector2::from((1.0, 0.0));
    let center = Vector2::from((0.0, 0.0));
    let angle = angle_between(&p1, &p2, &center, clockwise);
    assert_eq!(angle.to_degrees().round(), 270.0);

    // p2
    // |  90 degrees
    // c -- p1
    let p1 = Vector2::from((1.0, 0.0));
    let p2 = Vector2::from((0.0, 1.0));
    let center = Vector2::from((0.0, 0.0));
    let angle = angle_between(&p1, &p2, &center, clockwise);
    assert_eq!(angle.to_degrees().round(), 90.0);

    // c----p2
    //  \ 45 degrees
    //   p1
    let p1 = Vector2::from((1.0, -1.0));
    let p2 = Vector2::from((1.0, 0.0));
    let center = Vector2::from((0.0, 0.0));
    let angle = angle_between(&p1, &p2, &center, clockwise);
    assert_eq!(angle.to_degrees().round(), 45.0);

    //   c----p2
    //  / 135 degrees
    // p1
    let p1 = Vector2::from((-1.0, -1.0));
    let p2 = Vector2::from((1.0, 0.0));
    let center = Vector2::from((0.0, 0.0));
    let angle = angle_between(&p1, &p2, &center, clockwise);
    assert_eq!(angle.to_degrees().round(), 135.0);

    // p1
    //  \ 225 degrees
    //   c----p2
    let p1 = Vector2::from((-1.0, 1.0));
    let p2 = Vector2::from((1.0, 0.0));
    let center = Vector2::from((0.0, 0.0));
    let angle = angle_between(&p1, &p2, &center, clockwise);
    assert_eq!(angle.to_degrees().round(), 225.0);

    //   p1
    //  / 315 degrees
    // c----p2
    let p1 = Vector2::from((1.0, 1.0));
    let p2 = Vector2::from((1.0, 0.0));
    let center = Vector2::from((0.0, 0.0));
    let angle = angle_between(&p1, &p2, &center, clockwise);
    assert_eq!(angle.to_degrees().round(), 315.0);
}

#[test]
fn test_angle_between_cw() {
    let clockwise = true;
    // p1
    // |  90 degrees
    // c -- p2
    let p1 = Vector2::from((0.0, 1.0));
    let p2 = Vector2::from((1.0, 0.0));
    let center = Vector2::from((0.0, 0.0));
    let angle = angle_between(&p1, &p2, &center, clockwise);
    assert_eq!(angle.to_degrees().round(), 90.0);

    // p2
    // |  270 degrees
    // c -- p1
    let p1 = Vector2::from((1.0, 0.0));
    let p2 = Vector2::from((0.0, 1.0));
    let center = Vector2::from((0.0, 0.0));
    let angle = angle_between(&p1, &p2, &center, clockwise);
    assert_eq!(angle.to_degrees().round(), 270.0);

    // c----p2
    //  \ 315 degrees
    //   p1
    let p1 = Vector2::from((1.0, -1.0));
    let p2 = Vector2::from((1.0, 0.0));
    let center = Vector2::from((0.0, 0.0));
    let angle = angle_between(&p1, &p2, &center, clockwise);
    assert_eq!(angle.to_degrees().round(), 315.0);

    //   c----p2
    //  / 225 degrees
    // p1
    let p1 = Vector2::from((-1.0, -1.0));
    let p2 = Vector2::from((1.0, 0.0));
    let center = Vector2::from((0.0, 0.0));
    let angle = angle_between(&p1, &p2, &center, clockwise);
    assert_eq!(angle.to_degrees().round(), 225.0);

    // p1
    //  \ 135 degrees
    //   c----p2
    let p1 = Vector2::from((-1.0, 1.0));
    let p2 = Vector2::from((1.0, 0.0));
    let center = Vector2::from((0.0, 0.0));
    let angle = angle_between(&p1, &p2, &center, clockwise);
    assert_eq!(angle.to_degrees().round(), 135.0);

    //   p1
    //  / 45 degrees
    // c----p2
    let p1 = Vector2::from((1.0, 1.0));
    let p2 = Vector2::from((1.0, 0.0));
    let center = Vector2::from((0.0, 0.0));
    let angle = angle_between(&p1, &p2, &center, clockwise);
    assert_eq!(angle.to_degrees().round(), 45.0);
}

#[test]
fn correct_polygons() {
    let (verts, edges) = test_case_simple();
    let edges: Vec<Edge> = edges.keys().cloned().collect();
    let expected_polygons: Vec<Vec<i32>> = vec![vec![1, 2, 3, 0], vec![4, 0, 6, 5]];
    let actual_polygons = build_polygons(&edges, &verts);
    assert_eq!(expected_polygons, actual_polygons);
}
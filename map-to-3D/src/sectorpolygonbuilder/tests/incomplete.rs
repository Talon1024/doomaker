use super::*;

// see tests/data/incomplete.png for an annotated drawing of this data
fn test_case_incomplete() -> (Vec<Vec2>, Vec<Edge>) {
    let verts: Vec<Vec2> = vec![
        Vec2::new(3., 5.),
        Vec2::new(3., -5.),
        Vec2::new(-7., -5.), // Incomplete polygon end
        Vec2::new(-7., 5.),
        Vec2::new(-3., 5.),
        Vec2::new(-3., 2.),
        Vec2::new(-7., 2.), // Square end
    ];
    let edges: Vec<Edge> = vec![
        Edge::new(0, 1),
        Edge::new(1, 2),
        Edge::new(3, 4),
        Edge::new(4, 5),
        Edge::new(5, 6),
        Edge::new(6, 3),
    ];
    (verts, edges)
}

#[test]
fn correct_polygons() {
    let (verts, edges) = test_case_incomplete();
    let expected_polygons: Vec<SectorPolygon> = vec![
        SectorPolygon { vertices: vec![4, 5, 6, 3], hole_of: None }
    ];
    let actual_polygons = build_polygons(&edges, &verts);
    assert_eq!(expected_polygons, actual_polygons);
}

use super::*;

// see tests/data/harder.png for an annotated drawing of this data
fn test_case_harder() -> (Vec<Vec2>, Vec<Edge>) {
    let verts: Vec<Vec2> = vec![
        Vec2::new(64., 0.),      // 0
        Vec2::new(48., 48.),
        Vec2::new(48., -48.),
        Vec2::new(-48., 48.),
        Vec2::new(-64., 0.),     // 4
        Vec2::new(-48., -48.),
        Vec2::new(0., 64.),
        Vec2::new(0., -64.),
        Vec2::new(0., 48.),      // 8
        Vec2::new(48., 0.),
        Vec2::new(0., -48.),
        Vec2::new(-48., 0.),     // 11
    ];
    let edges: Vec<Edge> = vec![
        Edge::new(0, 1),
        Edge::new(0, 2),
        Edge::new(1, 9),
        Edge::new(9, 2), // Right polygon end
        Edge::new(1, 6),
        Edge::new(6, 3),
        Edge::new(3, 8),
        Edge::new(8, 1), // Upper polygon end
        Edge::new(2, 10),
        Edge::new(2, 7),
        Edge::new(7, 5),
        Edge::new(5, 10), // Lower polygon end
        Edge::new(5, 4),
        Edge::new(5, 11),
        Edge::new(4, 3),
        Edge::new(11, 3), // Left polygon end
        Edge::new(8, 9),
        Edge::new(9, 10),
        Edge::new(10, 11),
        Edge::new(11, 8), // Center polygon end
    ];
    (verts, edges)
}

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

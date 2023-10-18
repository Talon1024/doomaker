use super::*;

// See tests/data/holey.png for an illustration
fn test_case() -> (Vec<Vec2>, Vec<Edge>) {
    let verts: Vec<Vec2> = vec![
        Vec2::new(70., 30.),	// 0
        Vec2::new(68., 30.),
        Vec2::new(69., 32.),
        Vec2::new(64., 64.),	// 3
        Vec2::new(64., -64.),
        Vec2::new(-64., -64.),
        Vec2::new(-64., 64.),
        Vec2::new(44., 52.),	// 7
        Vec2::new(-52., 52.),
        Vec2::new(-52., -44.),
        Vec2::new(52., 44.),	// 10
        Vec2::new(52., -52.),
        Vec2::new(-44., -52.),	// 12
    ];
    let edges = vec![
        Edge::new(3, 4),	// 0
        Edge::new(4, 5),
        Edge::new(5, 6),
        Edge::new(6, 3),
        Edge::new(7, 8),	// 4
        Edge::new(8, 9),
        Edge::new(9, 7),
        Edge::new(10, 11),
        Edge::new(11, 12),	// 8
        Edge::new(12, 10),
        Edge::new(0, 1),
        Edge::new(1, 2),
        Edge::new(2, 0),	// 12
    ];
    (verts, edges)
}

#[test]
fn holey() {
    let (verts, edges) = test_case();
    let polys = build_polygons(&edges, &verts);
    let holes = [None, None, Some(1usize), Some(1)];
    assert_eq!(polys.iter().map(|p| p.hole_of).collect::<Vec<Option<usize>>>(), holes);

    // The triangulated polygons should have this many vertex indices
    let poly_lengths = [1 * 3, 12 * 3, 0, 0]; // The last two are holes
    // The vertex indices of the triangles should be within these ranges
    let poly_ranges = [0..=2, 3..=12, 0..=0, 0..=0]; // Last two are holes
    let triangulated = auto_triangulate(&polys, &verts);

    assert_eq!(triangulated.len(), poly_lengths.len());
    triangulated.iter().zip(poly_lengths).for_each(|(tp, l)| {
        if let Some(tp) = tp {
            assert_eq!(tp.len(), l);
        }
    });
    triangulated.iter().zip(poly_ranges).for_each(|(tp, r)| {
        if let Some(tp) = tp {
            assert!(tp.iter().all(|i| {r.contains(i)}))
        }
    }); 
}

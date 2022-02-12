use super::*;

// see tests/data/simple.png for an annotated drawing of this data
pub(super) fn test_case_simple() -> (Vec<MapVertex>, Vec<Edge>) {
    let verts: Vec<MapVertex> = vec![
        MapVertex { p: Vector2::from((0., 0.)) },
        MapVertex { p: Vector2::from((64., 0.)) },
        MapVertex { p: Vector2::from((64., -64.)) },
        MapVertex { p: Vector2::from((0., -64.)) },
        MapVertex { p: Vector2::from((0., 64.)) },
        MapVertex { p: Vector2::from((-64., 64.)) },
        MapVertex { p: Vector2::from((-64., 0.)) },
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

// see tests/data/harder.png for an annotated drawing of this data
pub(super) fn test_case_harder() -> (Vec<MapVertex>, Vec<Edge>) {
    let verts: Vec<MapVertex> = vec![
        MapVertex { p: Vector2::from((64., 0.)) },      // 0
        MapVertex { p: Vector2::from((48., 48.)) },
        MapVertex { p: Vector2::from((48., -48.)) },
        MapVertex { p: Vector2::from((-48., 48.)) },
        MapVertex { p: Vector2::from((-64., 0.)) },     // 4
        MapVertex { p: Vector2::from((-48., -48.)) },
        MapVertex { p: Vector2::from((0., 64.)) },
        MapVertex { p: Vector2::from((0., -64.)) },
        MapVertex { p: Vector2::from((0., 48.)) },      // 8
        MapVertex { p: Vector2::from((48., 0.)) },
        MapVertex { p: Vector2::from((0., -48.)) },
        MapVertex { p: Vector2::from((-48., 0.)) },     // 11
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


// see tests/data/insides.png for an annotated drawing of this data
pub(super) fn test_case_insides() -> (Vec<MapVertex>, Vec<Edge>) {
    let verts: Vec<MapVertex> = vec![
        MapVertex { p: Vector2::from((64., 64.)) },     // 0
        MapVertex { p: Vector2::from((64., 0.)) },
        MapVertex { p: Vector2::from((64., -64.)) },
        MapVertex { p: Vector2::from((0., -64.)) },
        MapVertex { p: Vector2::from((-64., -64.)) },       // 4
        MapVertex { p: Vector2::from((-64., 0.)) },
        MapVertex { p: Vector2::from((-64., 64.)) },
        MapVertex { p: Vector2::from((0., 64.)) },
        MapVertex { p: Vector2::from((99., 99.)) },       // 8
        MapVertex { p: Vector2::from((99., -99.)) },
        MapVertex { p: Vector2::from((-99., -99.)) },
        MapVertex { p: Vector2::from((-99., 99.)) },       // 11
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

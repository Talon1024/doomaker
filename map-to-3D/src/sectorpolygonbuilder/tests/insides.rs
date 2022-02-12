use super::*;
use data::test_case_insides;

#[test]
fn correct_polygons() {
    let (verts, edges) = test_case_insides();
    let expected_polygons: Vec<Vec<i32>> = vec![
        vec![8, 9, 10, 11],
        vec![1, 2, 3, 4, 5, 6, 7, 0],
        vec![7, 1, 3, 5],
    ];
    let expected_holes: Vec<Option<usize>> = vec![None, Some(0), None];
    let (actual_polygons, holes) = build_polygons(&edges, &verts);
    assert_eq!(expected_polygons, actual_polygons);
}

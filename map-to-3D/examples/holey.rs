#![warn(clippy::all)]
use map_to_3D::vector::Vector2;
use map_to_3D::edge::Edge;
use map_to_3D::sectorpolygonbuilder as spb;
use map_to_3D::plane::Plane;
use std::fs;

// See tests/data/holey.png for an illustration
fn test_case() -> (Vec<Vector2>, Vec<Edge>) {
	let verts: Vec<Vector2> = vec![
		Vector2::new(70., 30.),	// 0
		Vector2::new(68., 30.),
		Vector2::new(69., 32.),
		Vector2::new(64., 64.),	// 3
		Vector2::new(64., -64.),
		Vector2::new(-64., -64.),
		Vector2::new(-64., 64.),
		Vector2::new(44., 52.),	// 7
		Vector2::new(-52., 52.),
		Vector2::new(-52., -44.),
		Vector2::new(52., 44.),	// 10
		Vector2::new(52., -52.),
		Vector2::new(-44., -52.),	// 12
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

fn main() {
	let (verts, edges) = test_case();
	let polys = spb::build_polygons(&edges, &verts);

	let triangulated = spb::auto_triangulate(&polys, &verts);

	let secplane = Plane::Flat(0.);
	let normal = secplane.normal(false);
	let normal: String = normal.iter().map(|co| format!("{} ", co)).collect();
	let mut stl: String = String::from("solid holey");
	triangulated.iter().filter(|tp| tp.is_some()).for_each(|tp| {
		let tp = tp.as_ref().unwrap();
		tp.chunks_exact(3).for_each(|tri| {
			stl.push_str("\nfacet normal ");
			stl.push_str(normal.trim());
			stl.push_str("\n\touter loop");
			let a = verts[tri[0]];
			let a = format!("{} {} {}", a.x(), a.y(), secplane.z_at(&a));
			let b = verts[tri[1]];
			let b = format!("{} {} {}", b.x(), b.y(), secplane.z_at(&b));
			let c = verts[tri[2]];
			let c = format!("{} {} {}", c.x(), c.y(), secplane.z_at(&c));
			stl.push_str("\n\t\tvertex "); stl.push_str(&a);
			stl.push_str("\n\t\tvertex "); stl.push_str(&b);
			stl.push_str("\n\t\tvertex "); stl.push_str(&c);
			stl.push_str("\n\tendloop");
			stl.push_str("\nendfacet");
		});
	});
	stl.push_str("\nendsolid holey");
	match fs::write("holey.stl", stl) {
		Ok(()) => { println!("{:?}", fs::canonicalize("holey.stl")); }
		Err(e) => { println!("Failed to write holey.stl!\n{}", e); }
	};
}

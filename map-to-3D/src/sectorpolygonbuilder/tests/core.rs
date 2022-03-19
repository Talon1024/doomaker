use super::*;

#[test]
fn test_angle_between_ccw() {
	let clockwise = false;
	// p1
	// |  270 degrees
	// c -- p2
	let p1 = Vector2::new(0.0, 1.0);
	let p2 = Vector2::new(1.0, 0.0);
	let center = Vector2::new(0.0, 0.0);
	let angle = angle_between(&p1, &p2, &center, clockwise);
	assert_eq!(angle.to_degrees().round(), 270.0);

	// p2
	// |  90 degrees
	// c -- p1
	let p1 = Vector2::new(1.0, 0.0);
	let p2 = Vector2::new(0.0, 1.0);
	let center = Vector2::new(0.0, 0.0);
	let angle = angle_between(&p1, &p2, &center, clockwise);
	assert_eq!(angle.to_degrees().round(), 90.0);

	// c----p2
	//  \ 45 degrees
	//   p1
	let p1 = Vector2::new(1.0, -1.0);
	let p2 = Vector2::new(1.0, 0.0);
	let center = Vector2::new(0.0, 0.0);
	let angle = angle_between(&p1, &p2, &center, clockwise);
	assert_eq!(angle.to_degrees().round(), 45.0);

	//   c----p2
	//  / 135 degrees
	// p1
	let p1 = Vector2::new(-1.0, -1.0);
	let p2 = Vector2::new(1.0, 0.0);
	let center = Vector2::new(0.0, 0.0);
	let angle = angle_between(&p1, &p2, &center, clockwise);
	assert_eq!(angle.to_degrees().round(), 135.0);

	// p1
	//  \ 225 degrees
	//   c----p2
	let p1 = Vector2::new(-1.0, 1.0);
	let p2 = Vector2::new(1.0, 0.0);
	let center = Vector2::new(0.0, 0.0);
	let angle = angle_between(&p1, &p2, &center, clockwise);
	assert_eq!(angle.to_degrees().round(), 225.0);

	//   p1
	//  / 315 degrees
	// c----p2
	let p1 = Vector2::new(1.0, 1.0);
	let p2 = Vector2::new(1.0, 0.0);
	let center = Vector2::new(0.0, 0.0);
	let angle = angle_between(&p1, &p2, &center, clockwise);
	assert_eq!(angle.to_degrees().round(), 315.0);
}

#[test]
fn test_angle_between_cw() {
	let clockwise = true;
	// p1
	// |  90 degrees
	// c -- p2
	let p1 = Vector2::new(0.0, 1.0);
	let p2 = Vector2::new(1.0, 0.0);
	let center = Vector2::new(0.0, 0.0);
	let angle = angle_between(&p1, &p2, &center, clockwise);
	assert_eq!(angle.to_degrees().round(), 90.0);

	// p2
	// |  270 degrees
	// c -- p1
	let p1 = Vector2::new(1.0, 0.0);
	let p2 = Vector2::new(0.0, 1.0);
	let center = Vector2::new(0.0, 0.0);
	let angle = angle_between(&p1, &p2, &center, clockwise);
	assert_eq!(angle.to_degrees().round(), 270.0);

	// c----p2
	//  \ 315 degrees
	//   p1
	let p1 = Vector2::new(1.0, -1.0);
	let p2 = Vector2::new(1.0, 0.0);
	let center = Vector2::new(0.0, 0.0);
	let angle = angle_between(&p1, &p2, &center, clockwise);
	assert_eq!(angle.to_degrees().round(), 315.0);

	//   c----p2
	//  / 225 degrees
	// p1
	let p1 = Vector2::new(-1.0, -1.0);
	let p2 = Vector2::new(1.0, 0.0);
	let center = Vector2::new(0.0, 0.0);
	let angle = angle_between(&p1, &p2, &center, clockwise);
	assert_eq!(angle.to_degrees().round(), 225.0);

	// p1
	//  \ 135 degrees
	//   c----p2
	let p1 = Vector2::new(-1.0, 1.0);
	let p2 = Vector2::new(1.0, 0.0);
	let center = Vector2::new(0.0, 0.0);
	let angle = angle_between(&p1, &p2, &center, clockwise);
	assert_eq!(angle.to_degrees().round(), 135.0);

	//   p1
	//  / 45 degrees
	// c----p2
	let p1 = Vector2::new(1.0, 1.0);
	let p2 = Vector2::new(1.0, 0.0);
	let center = Vector2::new(0.0, 0.0);
	let angle = angle_between(&p1, &p2, &center, clockwise);
	assert_eq!(angle.to_degrees().round(), 45.0);
}

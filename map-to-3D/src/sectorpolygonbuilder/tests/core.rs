use super::*;
use std::f32::consts::*;

macro_rules! assert_gt {
    ($left:expr, $right:expr) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if left_val <= right_val {
                    panic!("left <= right (left: {:?}, right: {:?})", $left, $right)
                }
            }
        }
    };
}

macro_rules! assert_lt {
    ($left:expr, $right:expr) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if left_val >= right_val {
                    panic!("left >= right (left: {:?}, right: {:?})", $left, $right)
                }
            }
        }
    };
}

#[test]
fn test_angle_between_ccw() {
    let clockwise = false;
    // p1
    // |  270 degrees
    // c -- p2
    let p1 = Vec2::new(0.0, 1.0);
    let p2 = Vec2::new(1.0, 0.0);
    let center = Vec2::new(0.0, 0.0);
    let angle = angle_between(p1, p2, center, clockwise);
    assert_gt!(angle, Angle(FRAC_PI_2));
    assert_gt!(angle, Angle(PI));
    assert_lt!(angle, Angle(-0.));

    // p2
    // |  90 degrees
    // c -- p1
    let p1 = Vec2::new(1.0, 0.0);
    let p2 = Vec2::new(0.0, 1.0);
    let center = Vec2::new(0.0, 0.0);
    let angle = angle_between(p1, p2, center, clockwise);
    assert_lt!(angle, Angle(PI));

    // c----p2
    //  \ 45 degrees
    //   p1
    let p1 = Vec2::new(1.0, -1.0);
    let p2 = Vec2::new(1.0, 0.0);
    let center = Vec2::new(0.0, 0.0);
    let angle = angle_between(p1, p2, center, clockwise);
    assert_lt!(angle, Angle(FRAC_PI_2));

    //   c----p2
    //  / 135 degrees
    // p1
    let p1 = Vec2::new(-1.0, -1.0);
    let p2 = Vec2::new(1.0, 0.0);
    let center = Vec2::new(0.0, 0.0);
    let angle = angle_between(p1, p2, center, clockwise);
    assert_lt!(angle, Angle(PI));
    assert_gt!(angle, Angle(FRAC_PI_2));

    // p1
    //  \ 225 degrees
    //   c----p2
    let p1 = Vec2::new(-1.0, 1.0);
    let p2 = Vec2::new(1.0, 0.0);
    let center = Vec2::new(0.0, 0.0);
    let angle = angle_between(p1, p2, center, clockwise);
    assert_gt!(angle, Angle(PI));
    assert_lt!(angle, Angle(-FRAC_PI_2));

    //   p1
    //  / 315 degrees
    // c----p2
    let p1 = Vec2::new(1.0, 1.0);
    let p2 = Vec2::new(1.0, 0.0);
    let center = Vec2::new(0.0, 0.0);
    let angle = angle_between(p1, p2, center, clockwise);
    assert_gt!(angle, Angle(PI));
    assert_lt!(angle, Angle(-0.));
}

#[test]
fn test_angle_between_cw() {
    let clockwise = true;
    // p1
    // |  90 degrees
    // c -- p2
    let p1 = Vec2::new(0.0, 1.0);
    let p2 = Vec2::new(1.0, 0.0);
    let center = Vec2::new(0.0, 0.0);
    let angle = angle_between(p1, p2, center, clockwise);
    assert_lt!(angle, Angle(PI));
    assert_gt!(angle, Angle(0.));

    // p2
    // |  270 degrees
    // c -- p1
    let p1 = Vec2::new(1.0, 0.0);
    let p2 = Vec2::new(0.0, 1.0);
    let center = Vec2::new(0.0, 0.0);
    let angle = angle_between(p1, p2, center, clockwise);
    assert_gt!(angle, Angle(PI));
    assert_lt!(angle, Angle(-0.));

    // c----p2
    //  \ 315 degrees
    //   p1
    let p1 = Vec2::new(1.0, -1.0);
    let p2 = Vec2::new(1.0, 0.0);
    let center = Vec2::new(0.0, 0.0);
    let angle = angle_between(p1, p2, center, clockwise);
    assert_gt!(angle, Angle(PI));
    assert_lt!(angle, Angle(-0.));

    //   c----p2
    //  / 225 degrees
    // p1
    let p1 = Vec2::new(-1.0, -1.0);
    let p2 = Vec2::new(1.0, 0.0);
    let center = Vec2::new(0.0, 0.0);
    let angle = angle_between(p1, p2, center, clockwise);
    assert_gt!(angle, Angle(PI));
    assert_lt!(angle, Angle(-FRAC_PI_2));

    // p1
    //  \ 135 degrees
    //   c----p2
    let p1 = Vec2::new(-1.0, 1.0);
    let p2 = Vec2::new(1.0, 0.0);
    let center = Vec2::new(0.0, 0.0);
    let angle = angle_between(p1, p2, center, clockwise);
    assert_gt!(angle, Angle(FRAC_PI_2));
    assert_lt!(angle, Angle(PI));

    //   p1
    //  / 45 degrees
    // c----p2
    let p1 = Vec2::new(1.0, 1.0);
    let p2 = Vec2::new(1.0, 0.0);
    let center = Vec2::new(0.0, 0.0);
    let angle = angle_between(p1, p2, center, clockwise);
    assert_lt!(angle, Angle(FRAC_PI_2));
}

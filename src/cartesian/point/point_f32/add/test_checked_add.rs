use super::{checked_add, checked_add_assign, try_checked_add, try_checked_add_assign};
use crate::cartesian::point::point_f32::{MAX, MIN, PointF32};

#[test]
fn test() {
    assert_eq!(checked_add(&PointF32::of(0.0, 0.0), &PointF32::of(10.0, 13.0)), PointF32::of(10.0, 13.0));
    assert_eq!(checked_add(&PointF32::of(10.0, 13.0), &PointF32::of(-5.0, -3.0)), PointF32::of(5.0, 10.0));
}

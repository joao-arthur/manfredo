use super::saturating_add;
use crate::cartesian::point::point_f32::{MAX, MIN, PointF32};

#[test]
fn test() {
    assert_eq!(saturating_add(&PointF32::of(0.0, 0.0), &PointF32::of(10.0, 13.0)), PointF32::of(10.0, 13.0));
    assert_eq!(saturating_add(&PointF32::of(10.0, 10.0), &PointF32::of(-5.0, -3.0)), PointF32::of(5.0, 7.0));
}

#[test]
fn to_bounds() {
    assert_eq!(saturating_add(&PointF32::of(MIN + 2.0, MIN + 5.0), &PointF32::of(-2.0, -5.0)), PointF32::min());
    assert_eq!(saturating_add(&PointF32::of(MAX - 2.0, MAX - 5.0), &PointF32::of(2.0, 5.0)), PointF32::max());
}

#[test]
fn out_of_bounds() {
    assert_eq!(saturating_add(&PointF32::of(MIN + 2.0, MIN + 5.0), &PointF32::of(-10.0, -10.0)), PointF32::min());
    assert_eq!(saturating_add(&PointF32::of(MAX - 2.0, MAX - 5.0), &PointF32::of(10.0, 10.0)), PointF32::max());
}

#[test]
fn limits_out_of_bounds() {
    assert_eq!(saturating_add(&PointF32::of(MIN + 1.0, MIN + 1.0), &PointF32::min()), PointF32::min());
    assert_eq!(saturating_add(&PointF32::of(MAX - 1.0, MAX - 1.0), &PointF32::max()), PointF32::max());
}

use super::{wrapping_add, wrapping_add_assign};
use crate::cartesian::point::point_f64::{MAX, MIN, PointF64};

#[test]
fn test() {
    assert_eq!(wrapping_add(&PointF64::of(0.0, 0.0), &PointF64::of(10.0, 13.0)), PointF64::of(10.0, 13.0));
    assert_eq!(wrapping_add(&PointF64::of(10.0, 10.0), &PointF64::of(-5.0, -3.0)), PointF64::of(5.0, 7.0));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_add(&PointF64::of(MIN + 2.0, MIN + 5.0), &PointF64::of(-2.0, -5.0)), PointF64::min());
    assert_eq!(wrapping_add(&PointF64::of(MAX - 2.0, MAX - 5.0), &PointF64::of(2.0, 5.0)), PointF64::max());
}

#[test]
fn out_of_bounds() {
    assert_eq!(wrapping_add(&PointF64::of(MIN + 2.0, MIN + 5.0), &PointF64::of(-10.0, -10.0)), PointF64::of(MAX - 7.0, MAX - 4.0));
    assert_eq!(wrapping_add(&PointF64::of(MAX - 2.0, MAX - 5.0), &PointF64::of(10.0, 10.0)), PointF64::of(MIN + 7.0, MIN + 4.0));
}

#[test]
fn limits_out_of_bounds() {
    assert_eq!(wrapping_add(&PointF64::of(MIN + 1.0, MIN + 1.0), &PointF64::min()), PointF64::of(1.0, 1.0));
    assert_eq!(wrapping_add(&PointF64::of(MAX - 1.0, MAX - 1.0), &PointF64::max()), PointF64::of(-3.0, -3.0));
}

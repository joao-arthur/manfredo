use super::{saturating_add, saturating_add_assign};
use crate::cartesian::point::point_f32::{MAX, MIN, PointF32};

#[test]
fn test() {
    let mut p = PointF32::of(0.0, 0.0);
    saturating_add_assign(&mut p, &PointF32::of(10.0, 13.0));
    assert_eq!(p, PointF32::of(10.0, 13.0));
    saturating_add_assign(&mut p, &PointF32::of(-5.0, -3.0));
    assert_eq!(p, PointF32::of(5.0, 10.0));
}

#[test]
fn to_bounds() {
    let mut p_min = PointF32::of(MIN + 2.0, MIN + 5.0);
    saturating_add_assign(&mut p_min, &PointF32::of(-2.0, -5.0));
    assert_eq!(p_min, PointF32::min());

    let mut p_max = PointF32::of(MAX - 2.0, MAX - 5.0);
    saturating_add_assign(&mut p_max, &PointF32::of(2.0, 5.0));
    assert_eq!(p_max, PointF32::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = PointF32::of(MIN + 2.0, MIN + 5.0);
    saturating_add_assign(&mut p_min, &PointF32::of(-10.0, -10.0));
    assert_eq!(p_min, PointF32::min());

    let mut p_max = PointF32::of(MAX - 2.0, MAX - 5.0);
    saturating_add_assign(&mut p_max, &PointF32::of(10.0, 10.0));
    assert_eq!(p_max, PointF32::max());
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = PointF32::of(MIN + 1.0, MIN + 1.0);
    saturating_add_assign(&mut p_min, &PointF32::min());
    assert_eq!(p_min, PointF32::min());

    let mut p_max = PointF32::of(MAX - 1.0, MAX - 1.0);
    saturating_add_assign(&mut p_max, &PointF32::max());
    assert_eq!(p_max, PointF32::max());
}

use super::{saturating_add, saturating_add_assign};
use crate::cartesian::point::point_f32::{MAX, MIN, PointF32};

#[test]
fn test_saturating_add_assign() {
    let mut p = PointF32::of(0.0, 0.0);
    saturating_add_assign(&mut p, &PointF32::of(10.0, 13.0));
    assert_eq!(p, PointF32::of(10.0, 13.0));
    saturating_add_assign(&mut p, &PointF32::of(-5.0, -3.0));
    assert_eq!(p, PointF32::of(5.0, 10.0));
}

#[test]
fn saturating_add_assign_to_bounds() {
    let mut p_min = PointF32::of(MIN + 2.0, MIN + 5.0);
    saturating_add_assign(&mut p_min, &PointF32::of(-2.0, -5.0));
    assert_eq!(p_min, PointF32::min());

    let mut p_max = PointF32::of(MAX - 2.0, MAX - 5.0);
    saturating_add_assign(&mut p_max, &PointF32::of(2.0, 5.0));
    assert_eq!(p_max, PointF32::max());
}

#[test]
fn saturating_add_assign_out_of_bounds() {
    let mut p_min = PointF32::of(MIN + 2.0, MIN + 5.0);
    saturating_add_assign(&mut p_min, &PointF32::of(-10.0, -10.0));
    assert_eq!(p_min, PointF32::min());

    let mut p_max = PointF32::of(MAX - 2.0, MAX - 5.0);
    saturating_add_assign(&mut p_max, &PointF32::of(10.0, 10.0));
    assert_eq!(p_max, PointF32::max());
}

#[test]
fn saturating_add_assign_limits_out_of_bounds() {
    let mut p_min = PointF32::of(MIN + 1.0, MIN + 1.0);
    saturating_add_assign(&mut p_min, &PointF32::min());
    assert_eq!(p_min, PointF32::min());

    let mut p_max = PointF32::of(MAX - 1.0, MAX - 1.0);
    saturating_add_assign(&mut p_max, &PointF32::max());
    assert_eq!(p_max, PointF32::max());
}

#[test]
fn test_saturating_add() {
    assert_eq!(saturating_add(&PointF32::of(0.0, 0.0), &PointF32::of(10.0, 13.0)), PointF32::of(10.0, 13.0));
    assert_eq!(saturating_add(&PointF32::of(10.0, 10.0), &PointF32::of(-5.0, -3.0)), PointF32::of(5.0, 7.0));
}

#[test]
fn saturating_add_to_bounds() {
    assert_eq!(saturating_add(&PointF32::of(MIN + 2.0, MIN + 5.0), &PointF32::of(-2.0, -5.0)), PointF32::min());
    assert_eq!(saturating_add(&PointF32::of(MAX - 2.0, MAX - 5.0), &PointF32::of(2.0, 5.0)), PointF32::max());
}

#[test]
fn saturating_add_out_of_bounds() {
    assert_eq!(saturating_add(&PointF32::of(MIN + 2.0, MIN + 5.0), &PointF32::of(-10.0, -10.0)), PointF32::min());
    assert_eq!(saturating_add(&PointF32::of(MAX - 2.0, MAX - 5.0), &PointF32::of(10.0, 10.0)), PointF32::max());
}

#[test]
fn saturating_add_limits_out_of_bounds() {
    assert_eq!(saturating_add(&PointF32::of(MIN + 1.0, MIN + 1.0), &PointF32::min()), PointF32::min());
    assert_eq!(saturating_add(&PointF32::of(MAX - 1.0, MAX - 1.0), &PointF32::max()), PointF32::max());
}

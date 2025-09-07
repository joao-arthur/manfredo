use super::{saturating_add, saturating_add_assign};
use crate::matrix::point::point_i8::PointI8;

const MIN: i8 = i8::MIN;
const MAX: i8 = i8::MAX;

#[test]
fn test_saturating_add_assign() {
    let mut p = PointI8::of(0, 0);
    saturating_add_assign(&mut p, &PointI8::of(10, 13));
    assert_eq!(p, PointI8::of(10, 13));
    saturating_add_assign(&mut p, &PointI8::of(-5, -3));
    assert_eq!(p, PointI8::of(5, 10));
}

#[test]
fn saturating_add_assign_to_bounds() {
    let mut p_min = PointI8::of(MIN + 2, MIN + 5);
    saturating_add_assign(&mut p_min, &PointI8::of(-2, -5));
    assert_eq!(p_min, PointI8::min());

    let mut p_max = PointI8::of(MAX - 2, MAX - 5);
    saturating_add_assign(&mut p_max, &PointI8::of(2, 5));
    assert_eq!(p_max, PointI8::max());
}

#[test]
fn saturating_add_assign_out_of_bounds() {
    let mut p_min = PointI8::of(MIN + 2, MIN + 5);
    saturating_add_assign(&mut p_min, &PointI8::of(-10, -10));
    assert_eq!(p_min, PointI8::min());

    let mut p_max = PointI8::of(MAX - 2, MAX - 5);
    saturating_add_assign(&mut p_max, &PointI8::of(10, 10));
    assert_eq!(p_max, PointI8::max());
}

#[test]
fn saturating_add_assign_limits_out_of_bounds() {
    let mut p_min = PointI8::of(MIN + 1, MIN + 1);
    saturating_add_assign(&mut p_min, &PointI8::min());
    assert_eq!(p_min, PointI8::min());

    let mut p_max = PointI8::of(MAX - 1, MAX - 1);
    saturating_add_assign(&mut p_max, &PointI8::max());
    assert_eq!(p_max, PointI8::max());
}

#[test]
fn test_saturating_add() {
    assert_eq!(saturating_add(&PointI8::of(0, 0), &PointI8::of(10, 13)), PointI8::of(10, 13));
    assert_eq!(saturating_add(&PointI8::of(10, 10), &PointI8::of(-5, -3)), PointI8::of(5, 7));
}

#[test]
fn saturating_add_to_bounds() {
    assert_eq!(saturating_add(&PointI8::of(MIN + 2, MIN + 5), &PointI8::of(-2, -5)), PointI8::min());
    assert_eq!(saturating_add(&PointI8::of(MAX - 2, MAX - 5), &PointI8::of(2, 5)), PointI8::max());
}

#[test]
fn saturating_add_out_of_bounds() {
    assert_eq!(saturating_add(&PointI8::of(MIN + 2, MIN + 5), &PointI8::of(-10, -10)), PointI8::min());
    assert_eq!(saturating_add(&PointI8::of(MAX - 2, MAX - 5), &PointI8::of(10, 10)), PointI8::max());
}

#[test]
fn saturating_add_limits_out_of_bounds() {
    assert_eq!(saturating_add(&PointI8::of(MIN + 1, MIN + 1), &PointI8::min()), PointI8::min());
    assert_eq!(saturating_add(&PointI8::of(MAX - 1, MAX - 1), &PointI8::max()), PointI8::max());
}

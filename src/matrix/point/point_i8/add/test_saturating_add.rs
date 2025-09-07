use super::saturating_add;
use crate::matrix::point::point_i8::PointI8;

const MIN: i8 = i8::MIN;
const MAX: i8 = i8::MAX;

#[test]
fn test() {
    assert_eq!(saturating_add(&PointI8::of(0, 0), &PointI8::of(10, 13)), PointI8::of(10, 13));
    assert_eq!(saturating_add(&PointI8::of(10, 10), &PointI8::of(-5, -3)), PointI8::of(5, 7));
}

#[test]
fn to_bounds() {
    assert_eq!(saturating_add(&PointI8::of(MIN + 2, MIN + 5), &PointI8::of(-2, -5)), PointI8::min());
    assert_eq!(saturating_add(&PointI8::of(MAX - 2, MAX - 5), &PointI8::of(2, 5)), PointI8::max());
}

#[test]
fn out_of_bounds() {
    assert_eq!(saturating_add(&PointI8::of(MIN + 2, MIN + 5), &PointI8::of(-10, -10)), PointI8::min());
    assert_eq!(saturating_add(&PointI8::of(MAX - 2, MAX - 5), &PointI8::of(10, 10)), PointI8::max());
}

#[test]
fn limits_out_of_bounds() {
    assert_eq!(saturating_add(&PointI8::of(MIN + 1, MIN + 1), &PointI8::min()), PointI8::min());
    assert_eq!(saturating_add(&PointI8::of(MAX - 1, MAX - 1), &PointI8::max()), PointI8::max());
}

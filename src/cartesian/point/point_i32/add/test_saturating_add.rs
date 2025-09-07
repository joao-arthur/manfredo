use super::saturating_add;
use crate::cartesian::point::point_i32::PointI32;

const MIN: i32 = i32::MIN;
const MAX: i32 = i32::MAX;

#[test]
fn test() {
    assert_eq!(saturating_add(&PointI32::of(0, 0), &PointI32::of(10, 13)), PointI32::of(10, 13));
    assert_eq!(saturating_add(&PointI32::of(10, 10), &PointI32::of(-5, -3)), PointI32::of(5, 7));
}

#[test]
fn to_bounds() {
    assert_eq!(saturating_add(&PointI32::of(MIN + 2, MIN + 5), &PointI32::of(-2, -5)), PointI32::min());
    assert_eq!(saturating_add(&PointI32::of(MAX - 2, MAX - 5), &PointI32::of(2, 5)), PointI32::max());
}

#[test]
fn out_of_bounds() {
    assert_eq!(saturating_add(&PointI32::of(MIN + 2, MIN + 5), &PointI32::of(-10, -10)), PointI32::min());
    assert_eq!(saturating_add(&PointI32::of(MAX - 2, MAX - 5), &PointI32::of(10, 10)), PointI32::max());
}

#[test]
fn limits_out_of_bounds() {
    assert_eq!(saturating_add(&PointI32::of(MIN + 1, MIN + 1), &PointI32::min()), PointI32::min());
    assert_eq!(saturating_add(&PointI32::of(MAX - 1, MAX - 1), &PointI32::max()), PointI32::max());
}

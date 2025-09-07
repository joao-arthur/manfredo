use super::saturating_add;
use crate::cartesian::point::point_i64::PointI64;

const MIN: i64 = i64::MIN;
const MAX: i64 = i64::MAX;

#[test]
fn test() {
    assert_eq!(saturating_add(&PointI64::of(0, 0), &PointI64::of(10, 13)), PointI64::of(10, 13));
    assert_eq!(saturating_add(&PointI64::of(10, 10), &PointI64::of(-5, -3)), PointI64::of(5, 7));
}

#[test]
fn to_bounds() {
    assert_eq!(saturating_add(&PointI64::of(MIN + 2, MIN + 5), &PointI64::of(-2, -5)), PointI64::min());
    assert_eq!(saturating_add(&PointI64::of(MAX - 2, MAX - 5), &PointI64::of(2, 5)), PointI64::max());
}

#[test]
fn out_of_bounds() {
    assert_eq!(saturating_add(&PointI64::of(MIN + 2, MIN + 5), &PointI64::of(-10, -10)), PointI64::min());
    assert_eq!(saturating_add(&PointI64::of(MAX - 2, MAX - 5), &PointI64::of(10, 10)), PointI64::max());
}

#[test]
fn limits_out_of_bounds() {
    assert_eq!(saturating_add(&PointI64::of(MIN + 1, MIN + 1), &PointI64::min()), PointI64::min());
    assert_eq!(saturating_add(&PointI64::of(MAX - 1, MAX - 1), &PointI64::max()), PointI64::max());
}

use super::saturating_add;
use crate::cartesian::point::{point_i64::PointI64 as PointI, point_u64::PointU64};

const MAX: u64 = u64::MAX;

#[test]
fn test() {
    assert_eq!(saturating_add(&PointU64::min(), &PointI::of(10, 13)), PointU64::of(10, 13));
    assert_eq!(saturating_add(&PointU64::of(10, 10), &PointI::of(-5, -3)), PointU64::of(5, 7));
}

#[test]
fn to_bounds() {
    assert_eq!(saturating_add(&PointU64::of(2, 5), &PointI::of(-2, -5)), PointU64::min());
    assert_eq!(saturating_add(&PointU64::of(MAX - 2, MAX - 5), &PointI::of(2, 5)), PointU64::max());
}

#[test]
fn out_of_bounds() {
    assert_eq!(saturating_add(&PointU64::of(2, 5), &PointI::of(-10, -10)), PointU64::min());
    assert_eq!(saturating_add(&PointU64::of(MAX - 2, MAX - 5), &PointI::of(10, 10)), PointU64::max());
}

#[test]
fn limits_out_of_bounds() {
    assert_eq!(saturating_add(&PointU64::of(1, 1), &PointI::min()), PointU64::min());
    assert_eq!(saturating_add(&PointU64::of(MAX - 1, MAX - 1), &PointI::max()), PointU64::max());
}

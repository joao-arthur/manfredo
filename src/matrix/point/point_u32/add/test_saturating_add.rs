use super::saturating_add;
use crate::matrix::point::{point_i32::PointI32 as PointI, point_u32::PointU32};

const MAX: u32 = u32::MAX;

#[test]
fn test() {
    assert_eq!(saturating_add(&PointU32::min(), &PointI::of(10, 13)), PointU32::of(10, 13));
    assert_eq!(saturating_add(&PointU32::of(10, 10), &PointI::of(-5, -3)), PointU32::of(5, 7));
}

#[test]
fn to_bounds() {
    assert_eq!(saturating_add(&PointU32::of(2, 5), &PointI::of(-2, -5)), PointU32::min());
    assert_eq!(saturating_add(&PointU32::of(MAX - 2, MAX - 5), &PointI::of(2, 5)), PointU32::max());
}

#[test]
fn out_of_bounds() {
    assert_eq!(saturating_add(&PointU32::of(2, 5), &PointI::of(-10, -10)), PointU32::min());
    assert_eq!(saturating_add(&PointU32::of(MAX - 2, MAX - 5), &PointI::of(10, 10)), PointU32::max());
}

#[test]
fn limits_out_of_bounds() {
    assert_eq!(saturating_add(&PointU32::of(1, 1), &PointI::min()), PointU32::min());
    assert_eq!(saturating_add(&PointU32::of(MAX - 1, MAX - 1), &PointI::max()), PointU32::max());
}

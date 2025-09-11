use super::wrapping_add;
use crate::cartesian::point::{point_i16::PointI16 as PointI, point_u16::PointU16};

const MAX: u16 = u16::MAX;

#[test]
fn test() {
    assert_eq!(wrapping_add(&PointU16::min(), &PointI::of(10, 13)), PointU16::of(10, 13));
    assert_eq!(wrapping_add(&PointU16::of(10, 10), &PointI::of(-5, -3)), PointU16::of(5, 7));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_add(&PointU16::of(2, 5), &PointI::of(-2, -5)), PointU16::min());
    assert_eq!(wrapping_add(&PointU16::of(MAX - 2, MAX - 5), &PointI::of(2, 5)), PointU16::max());
}

#[test]
fn out_of_bounds() {
    assert_eq!(wrapping_add(&PointU16::of(2, 5), &PointI::of(-10, -10)), PointU16::of(MAX - 7, MAX - 4));
    assert_eq!(wrapping_add(&PointU16::of(MAX - 2, MAX - 5), &PointI::of(10, 10)), PointU16::of(7, 4));
}

#[test]
fn limits_out_of_bounds() {
    assert_eq!(wrapping_add(&PointU16::of(1, 1), &PointI::min()), PointU16::of(32769, 32769));
    assert_eq!(wrapping_add(&PointU16::of(MAX - 1, MAX - 1), &PointI::max()), PointU16::of(32765, 32765));
}

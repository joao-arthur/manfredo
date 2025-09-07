use super::wrapping_add;
use crate::cartesian::point::{point_i8::PointI8, point_u8::PointU8};

const MAX: u8 = u8::MAX;

#[test]
fn test() {
    assert_eq!(wrapping_add(&PointU8::min(), &PointI8::of(10, 13)), PointU8::of(10, 13));
    assert_eq!(wrapping_add(&PointU8::of(10, 10), &PointI8::of(-5, -3)), PointU8::of(5, 7));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_add(&PointU8::of(2, 5), &PointI8::of(-2, -5)), PointU8::min());
    assert_eq!(wrapping_add(&PointU8::of(MAX - 2, MAX - 5), &PointI8::of(2, 5)), PointU8::max());
}

#[test]
fn out_of_bounds() {
    assert_eq!(wrapping_add(&PointU8::of(2, 5), &PointI8::of(-10, -10)), PointU8::of(MAX - 7, MAX - 4));
    assert_eq!(wrapping_add(&PointU8::of(MAX - 2, MAX - 5), &PointI8::of(10, 10)), PointU8::of(7, 4));
}

#[test]
fn limits_out_of_bounds() {
    assert_eq!(wrapping_add(&PointU8::of(1, 1), &PointI8::min()), PointU8::of(129, 129));
    assert_eq!(wrapping_add(&PointU8::of(MAX - 1, MAX - 1), &PointI8::max()), PointU8::of(125, 125));
}

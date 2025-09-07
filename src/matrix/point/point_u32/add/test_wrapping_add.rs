use super::wrapping_add;
use crate::matrix::point::{point_i32::PointI32, point_u32::PointU32};

const MAX: u32 = u32::MAX;

#[test]
fn test() {
    assert_eq!(wrapping_add(&PointU32::min(), &PointI32::of(10, 13)), PointU32::of(10, 13));
    assert_eq!(wrapping_add(&PointU32::of(10, 10), &PointI32::of(-5, -3)), PointU32::of(5, 7));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_add(&PointU32::of(2, 5), &PointI32::of(-2, -5)), PointU32::min());
    assert_eq!(wrapping_add(&PointU32::of(MAX - 2, MAX - 5), &PointI32::of(2, 5)), PointU32::max());
}

#[test]
fn out_of_bounds() {
    assert_eq!(wrapping_add(&PointU32::of(2, 5), &PointI32::of(-10, -10)), PointU32::of(MAX - 7, MAX - 4));
    assert_eq!(wrapping_add(&PointU32::of(MAX - 2, MAX - 5), &PointI32::of(10, 10)), PointU32::of(7, 4));
}

#[test]
fn limits_out_of_bounds() {
    assert_eq!(wrapping_add(&PointU32::of(1, 1), &PointI32::min()), PointU32::of(2147483649, 2147483649));
    assert_eq!(wrapping_add(&PointU32::of(MAX - 1, MAX - 1), &PointI32::max()), PointU32::of(2147483645, 2147483645));
}

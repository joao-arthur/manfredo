use super::wrapping_add;
use crate::cartesian::d1::point::{point_i64::Point as PointI, point_u64::Point};

const MAX: u64 = u64::MAX;

#[test]
fn test() {
    assert_eq!(wrapping_add(&Point::min(), &PointI::of(10)), Point::of(10));
    assert_eq!(wrapping_add(&Point::of(10), &PointI::of(-5)), Point::of(5));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_add(&Point::of(2), &PointI::of(-2)), Point::min());
    assert_eq!(wrapping_add(&Point::of(MAX - 2), &PointI::of(2)), Point::max());
}

#[test]
fn out_of_bounds() {
    assert_eq!(wrapping_add(&Point::of(2), &PointI::of(-10)), Point::of(MAX - 7));
    assert_eq!(wrapping_add(&Point::of(MAX - 2), &PointI::of(10)), Point::of(7));
}

#[test]
fn limits_out_of_bounds() {
    assert_eq!(wrapping_add(&Point::of(1), &PointI::min()), Point::of(9223372036854775809));
    assert_eq!(wrapping_add(&Point::of(MAX - 1), &PointI::max()), Point::of(9223372036854775805));
}

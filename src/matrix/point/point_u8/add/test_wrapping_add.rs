use super::wrapping_add;
use crate::matrix::point::{point_i8::Point as PointI, point_u8::Point};

const MAX: u8 = u8::MAX;

#[test]
fn test() {
    assert_eq!(wrapping_add(&Point::min(), &PointI::of(10, 13)), Point::of(10, 13));
    assert_eq!(wrapping_add(&Point::of(10, 10), &PointI::of(-5, -3)), Point::of(5, 7));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_add(&Point::of(2, 5), &PointI::of(-2, -5)), Point::min());
    assert_eq!(wrapping_add(&Point::of(MAX - 2, MAX - 5), &PointI::of(2, 5)), Point::max());
}

#[test]
fn out_of_bounds() {
    assert_eq!(wrapping_add(&Point::of(2, 5), &PointI::of(-10, -10)), Point::of(MAX - 7, MAX - 4));
    assert_eq!(wrapping_add(&Point::of(MAX - 2, MAX - 5), &PointI::of(10, 10)), Point::of(7, 4));
}

#[test]
fn limits_out_of_bounds() {
    assert_eq!(wrapping_add(&Point::of(1, 1), &PointI::min()), Point::of(129, 129));
    assert_eq!(wrapping_add(&Point::of(MAX - 1, MAX - 1), &PointI::max()), Point::of(125, 125));
}

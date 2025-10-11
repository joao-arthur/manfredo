use super::wrapping_add;
use crate::cartesian::d1::point::point_i8::Point;

const MIN: i8 = i8::MIN;
const MAX: i8 = i8::MAX;

#[test]
fn test() {
    assert_eq!(wrapping_add(&Point::of(0), &Point::of(10)), Point::of(10));
    assert_eq!(wrapping_add(&Point::of(10), &Point::of(-5)), Point::of(5));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_add(&Point::of(MIN + 2), &Point::of(-2)), Point::min());
    assert_eq!(wrapping_add(&Point::of(MAX - 2), &Point::of(2)), Point::max());
}

#[test]
fn out_of_bounds() {
    assert_eq!(wrapping_add(&Point::of(MIN + 2), &Point::of(-10)), Point::of(MAX - 7));
    assert_eq!(wrapping_add(&Point::of(MAX - 2), &Point::of(10)), Point::of(MIN + 7));
}

#[test]
fn limits_out_of_bounds() {
    assert_eq!(wrapping_add(&Point::of(MIN + 1), &Point::min()), Point::of(1));
    assert_eq!(wrapping_add(&Point::of(MAX - 1), &Point::max()), Point::of(-3));
}

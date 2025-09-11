use super::wrapping_add;
use crate::cartesian::point::point_i64::Point;

const MIN: i64 = i64::MIN;
const MAX: i64 = i64::MAX;

#[test]
fn test() {
    assert_eq!(wrapping_add(&Point::of(0, 0), &Point::of(10, 13)), Point::of(10, 13));
    assert_eq!(wrapping_add(&Point::of(10, 10), &Point::of(-5, -3)), Point::of(5, 7));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_add(&Point::of(MIN + 2, MIN + 5), &Point::of(-2, -5)), Point::min());
    assert_eq!(wrapping_add(&Point::of(MAX - 2, MAX - 5), &Point::of(2, 5)), Point::max());
}

#[test]
fn out_of_bounds() {
    assert_eq!(wrapping_add(&Point::of(MIN + 2, MIN + 5), &Point::of(-10, -10)), Point::of(MAX - 7, MAX - 4));
    assert_eq!(wrapping_add(&Point::of(MAX - 2, MAX - 5), &Point::of(10, 10)), Point::of(MIN + 7, MIN + 4));
}

#[test]
fn limits_out_of_bounds() {
    assert_eq!(wrapping_add(&Point::of(MIN + 1, MIN + 1), &Point::min()), Point::of(1, 1));
    assert_eq!(wrapping_add(&Point::of(MAX - 1, MAX - 1), &Point::max()), Point::of(-3, -3));
}

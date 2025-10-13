use super::saturating_add;
use crate::matrix::d1::point::point_i32::Point;

const MIN: i32 = i32::MIN;
const MAX: i32 = i32::MAX;

#[test]
fn test() {
    assert_eq!(saturating_add(&Point::of(0), &Point::of(10)), Point::of(10));
    assert_eq!(saturating_add(&Point::of(10), &Point::of(-5)), Point::of(5));
}

#[test]
fn to_bounds() {
    assert_eq!(saturating_add(&Point::of(MIN + 2), &Point::of(-2)), Point::min());
    assert_eq!(saturating_add(&Point::of(MAX - 2), &Point::of(2)), Point::max());
}

#[test]
fn out_of_bounds() {
    assert_eq!(saturating_add(&Point::of(MIN + 2), &Point::of(-10)), Point::min());
    assert_eq!(saturating_add(&Point::of(MAX - 2), &Point::of(10)), Point::max());
}

#[test]
fn limits_out_of_bounds() {
    assert_eq!(saturating_add(&Point::of(MIN + 1), &Point::min()), Point::min());
    assert_eq!(saturating_add(&Point::of(MAX - 1), &Point::max()), Point::max());
}

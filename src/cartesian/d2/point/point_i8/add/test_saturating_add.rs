use super::saturating_add;
use crate::cartesian::d2::point::point_i8::Point;

const MIN: i8 = i8::MIN;
const MAX: i8 = i8::MAX;

#[test]
fn test() {
    assert_eq!(saturating_add(&Point::of(0, 0), &Point::of(10, 13)), Point::of(10, 13));
    assert_eq!(saturating_add(&Point::of(10, 10), &Point::of(-5, -3)), Point::of(5, 7));
}

#[test]
fn to_bounds() {
    assert_eq!(saturating_add(&Point::of(MIN + 2, MIN + 5), &Point::of(-2, -5)), Point::min());
    assert_eq!(saturating_add(&Point::of(MAX - 2, MAX - 5), &Point::of(2, 5)), Point::max());
}

#[test]
fn out_of_bounds() {
    assert_eq!(saturating_add(&Point::of(MIN + 2, MIN + 5), &Point::of(-10, -10)), Point::min());
    assert_eq!(saturating_add(&Point::of(MAX - 2, MAX - 5), &Point::of(10, 10)), Point::max());
}

#[test]
fn limits_out_of_bounds() {
    assert_eq!(saturating_add(&Point::of(MIN + 1, MIN + 1), &Point::min()), Point::min());
    assert_eq!(saturating_add(&Point::of(MAX - 1, MAX - 1), &Point::max()), Point::max());
}

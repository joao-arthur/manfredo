use super::saturating_add;
use crate::cartesian::d1::point::point_i16::{MAX, MIN, Point};

#[test]
fn test() {
    assert_eq!(saturating_add(&Point::zero(), &Point::new(10)), Point::new(10));
    assert_eq!(saturating_add(&Point::new(10), &Point::new(-5)), Point::new(5));
}

#[test]
fn to_bounds() {
    assert_eq!(saturating_add(&Point::new(MIN + 2), &Point::new(-2)), Point::min());
    assert_eq!(saturating_add(&Point::new(MAX - 2), &Point::new(2)), Point::max());
}

#[test]
fn out_of_bounds() {
    assert_eq!(saturating_add(&Point::new(MIN + 2), &Point::new(-10)), Point::min());
    assert_eq!(saturating_add(&Point::new(MAX - 2), &Point::new(10)), Point::max());
}

#[test]
fn limits_out_of_bounds() {
    assert_eq!(saturating_add(&Point::new(MIN + 1), &Point::min()), Point::min());
    assert_eq!(saturating_add(&Point::new(MAX - 1), &Point::max()), Point::max());
}

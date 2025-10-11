use super::saturating_add;
use crate::cartesian::d1::point::point_f32::{MAX, MIN, Point};

#[test]
fn test() {
    assert_eq!(saturating_add(&Point::of(0.0), &Point::of(10.0)), Point::of(10.0));
    assert_eq!(saturating_add(&Point::of(10.0), &Point::of(-5.0)), Point::of(5.0));
}

#[test]
fn to_bounds() {
    assert_eq!(saturating_add(&Point::of(MIN + 2.0), &Point::of(-2.0)), Point::min());
    assert_eq!(saturating_add(&Point::of(MAX - 2.0), &Point::of(2.0)), Point::max());
}

#[test]
fn out_of_bounds() {
    assert_eq!(saturating_add(&Point::of(MIN + 2.0), &Point::of(-10.0)), Point::min());
    assert_eq!(saturating_add(&Point::of(MAX - 2.0), &Point::of(10.0)), Point::max());
}

#[test]
fn limits_out_of_bounds() {
    assert_eq!(saturating_add(&Point::of(MIN + 1.0), &Point::min()), Point::min());
    assert_eq!(saturating_add(&Point::of(MAX - 1.0), &Point::max()), Point::max());
}

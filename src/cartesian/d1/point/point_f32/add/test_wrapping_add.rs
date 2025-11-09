use super::wrapping_add;
use crate::cartesian::d1::point::point_f32::{MAX, MIN, Point};

#[test]
fn test() {
    assert_eq!(wrapping_add(&Point::zero(), &Point::new(10.0)), Point::new(10.0));
    assert_eq!(wrapping_add(&Point::new(10.0), &Point::new(-5.0)), Point::new(5.0));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_add(&Point::new(MIN + 2.0), &Point::new(-2.0)), Point::min());
    assert_eq!(wrapping_add(&Point::new(MAX - 2.0), &Point::new(2.0)), Point::max());
}

#[test]
fn out_of_bounds() {
    assert_eq!(wrapping_add(&Point::new(MIN + 2.0), &Point::new(-10.0)), Point::new(MAX - 7.0));
    assert_eq!(wrapping_add(&Point::new(MAX - 2.0), &Point::new(10.0)), Point::new(MIN + 7.0));
}

#[test]
fn limits_out_of_bounds() {
    assert_eq!(wrapping_add(&Point::new(MIN + 1.0), &Point::min()), Point::new(1.0));
    assert_eq!(wrapping_add(&Point::new(MAX - 1.0), &Point::max()), Point::new(-3.0));
}

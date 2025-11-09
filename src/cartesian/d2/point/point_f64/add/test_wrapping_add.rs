use super::wrapping_add;
use crate::cartesian::{
    d1::point::point_f64::{MAX, MIN},
    d2::point::point_f64::Point,
};

#[test]
fn test() {
    assert_eq!(wrapping_add(&Point::zero(), &Point::new(10.0, 13.0)), Point::new(10.0, 13.0));
    assert_eq!(wrapping_add(&Point::new(10.0, 10.0), &Point::new(-5.0, -3.0)), Point::new(5.0, 7.0));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_add(&Point::new(MIN + 2.0, MIN + 5.0), &Point::new(-2.0, -5.0)), Point::min());
    assert_eq!(wrapping_add(&Point::new(MAX - 2.0, MAX - 5.0), &Point::new(2.0, 5.0)), Point::max());
}

#[test]
fn out_of_bounds() {
    assert_eq!(wrapping_add(&Point::new(MIN + 2.0, MIN + 5.0), &Point::new(-10.0, -10.0)), Point::new(MAX - 7.0, MAX - 4.0));
    assert_eq!(wrapping_add(&Point::new(MAX - 2.0, MAX - 5.0), &Point::new(10.0, 10.0)), Point::new(MIN + 7.0, MIN + 4.0));
}

#[test]
fn limits_out_of_bounds() {
    assert_eq!(wrapping_add(&Point::new(MIN + 1.0, MIN + 1.0), &Point::min()), Point::new(1.0, 1.0));
    assert_eq!(wrapping_add(&Point::new(MAX - 1.0, MAX - 1.0), &Point::max()), Point::new(-3.0, -3.0));
}

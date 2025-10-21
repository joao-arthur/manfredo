use super::wrapping_add;
use crate::cartesian::{
    d1::point::point_f32::{MAX, MIN},
    d2::point::point_f32::Point,
};

#[test]
fn test() {
    assert_eq!(wrapping_add(&Point::zero(), &Point::of(10.0, 13.0)), Point::of(10.0, 13.0));
    assert_eq!(wrapping_add(&Point::of(10.0, 10.0), &Point::of(-5.0, -3.0)), Point::of(5.0, 7.0));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_add(&Point::of(MIN + 2.0, MIN + 5.0), &Point::of(-2.0, -5.0)), Point::min());
    assert_eq!(wrapping_add(&Point::of(MAX - 2.0, MAX - 5.0), &Point::of(2.0, 5.0)), Point::max());
}

#[test]
fn out_of_bounds() {
    assert_eq!(wrapping_add(&Point::of(MIN + 2.0, MIN + 5.0), &Point::of(-10.0, -10.0)), Point::of(MAX - 7.0, MAX - 4.0));
    assert_eq!(wrapping_add(&Point::of(MAX - 2.0, MAX - 5.0), &Point::of(10.0, 10.0)), Point::of(MIN + 7.0, MIN + 4.0));
}

#[test]
fn limits_out_of_bounds() {
    assert_eq!(wrapping_add(&Point::of(MIN + 1.0, MIN + 1.0), &Point::min()), Point::of(1.0, 1.0));
    assert_eq!(wrapping_add(&Point::of(MAX - 1.0, MAX - 1.0), &Point::max()), Point::of(-3.0, -3.0));
}

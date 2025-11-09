use super::saturating_add;
use crate::cartesian::{
    d1::point::point_f32::{MAX, MIN},
    d2::point::point_f32::Point,
};

#[test]
fn test() {
    assert_eq!(saturating_add(&Point::zero(), &Point::new(10.0, 13.0)), Point::new(10.0, 13.0));
    assert_eq!(saturating_add(&Point::new(10.0, 10.0), &Point::new(-5.0, -3.0)), Point::new(5.0, 7.0));
}

#[test]
fn to_bounds() {
    assert_eq!(saturating_add(&Point::new(MIN + 2.0, MIN + 5.0), &Point::new(-2.0, -5.0)), Point::min());
    assert_eq!(saturating_add(&Point::new(MAX - 2.0, MAX - 5.0), &Point::new(2.0, 5.0)), Point::max());
}

#[test]
fn out_of_bounds() {
    assert_eq!(saturating_add(&Point::new(MIN + 2.0, MIN + 5.0), &Point::new(-10.0, -10.0)), Point::min());
    assert_eq!(saturating_add(&Point::new(MAX - 2.0, MAX - 5.0), &Point::new(10.0, 10.0)), Point::max());
}

#[test]
fn limits_out_of_bounds() {
    assert_eq!(saturating_add(&Point::new(MIN + 1.0, MIN + 1.0), &Point::min()), Point::min());
    assert_eq!(saturating_add(&Point::new(MAX - 1.0, MAX - 1.0), &Point::max()), Point::max());
}

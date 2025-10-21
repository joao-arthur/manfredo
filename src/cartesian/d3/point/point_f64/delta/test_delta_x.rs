use super::delta_x;
use crate::cartesian::d1::point::point_f64::{MAX, MIN};
use crate::cartesian::d3::point::point_f64::Point;

#[test]
fn from_min() {
    assert_eq!(delta_x(&Point::of(MIN, 0.0, 0.0), &Point::of(-1.0, 0.0, 0.0)), MAX);
    assert_eq!(delta_x(&Point::of(0.0, MIN, 0.0), &Point::of(0.0, -1.0, 0.0)), 0.0);
    assert_eq!(delta_x(&Point::of(0.0, 0.0, MIN), &Point::of(0.0, 0.0, -1.0)), 0.0);
}

#[test]
fn to_max() {
    assert_eq!(delta_x(&Point::zero(), &Point::of(MAX, 0.0, 0.0)), MAX);
    assert_eq!(delta_x(&Point::zero(), &Point::of(0.0, MAX, 0.0)), 0.0);
    assert_eq!(delta_x(&Point::zero(), &Point::of(0.0, 0.0, MAX)), 0.0);
}

#[test]
fn to_min() {
    assert_eq!(delta_x(&Point::zero(), &Point::of(MIN, 0.0, 0.0)), MIN);
    assert_eq!(delta_x(&Point::zero(), &Point::of(0.0, MIN, 0.0)), 0.0);
    assert_eq!(delta_x(&Point::zero(), &Point::of(0.0, 0.0, MIN)), 0.0);
}

#[test]
fn from_max() {
    assert_eq!(delta_x(&Point::of(MAX, 0.0, 0.0), &Point::zero()), MIN + 1.0);
    assert_eq!(delta_x(&Point::of(0.0, MAX, 0.0), &Point::zero()), 0.0);
    assert_eq!(delta_x(&Point::of(0.0, 0.0, MAX), &Point::zero()), 0.0);
}

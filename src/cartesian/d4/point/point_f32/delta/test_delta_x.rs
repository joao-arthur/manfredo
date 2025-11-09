use super::delta_x;
use crate::cartesian::{
    d1::point::point_f32::{MAX, MIN},
    d4::point::point_f32::Point,
};

#[test]
fn from_min() {
    assert_eq!(delta_x(&Point::new(MIN, 0.0, 0.0, 0.0), &Point::new(-1.0, 0.0, 0.0, 0.0)), MAX);
    assert_eq!(delta_x(&Point::new(0.0, MIN, 0.0, 0.0), &Point::new(0.0, -1.0, 0.0, 0.0)), 0.0);
    assert_eq!(delta_x(&Point::new(0.0, 0.0, MIN, 0.0), &Point::new(0.0, 0.0, -1.0, 0.0)), 0.0);
    assert_eq!(delta_x(&Point::new(0.0, 0.0, 0.0, MIN), &Point::new(0.0, 0.0, 0.0, -1.0)), 0.0);
}

#[test]
fn to_max() {
    assert_eq!(delta_x(&Point::zero(), &Point::new(MAX, 0.0, 0.0, 0.0)), MAX);
    assert_eq!(delta_x(&Point::zero(), &Point::new(0.0, MAX, 0.0, 0.0)), 0.0);
    assert_eq!(delta_x(&Point::zero(), &Point::new(0.0, 0.0, MAX, 0.0)), 0.0);
    assert_eq!(delta_x(&Point::zero(), &Point::new(0.0, 0.0, 0.0, MAX)), 0.0);
}

#[test]
fn to_min() {
    assert_eq!(delta_x(&Point::zero(), &Point::new(MIN, 0.0, 0.0, 0.0)), MIN);
    assert_eq!(delta_x(&Point::zero(), &Point::new(0.0, MIN, 0.0, 0.0)), 0.0);
    assert_eq!(delta_x(&Point::zero(), &Point::new(0.0, 0.0, MIN, 0.0)), 0.0);
    assert_eq!(delta_x(&Point::zero(), &Point::new(0.0, 0.0, 0.0, MIN)), 0.0);
}

#[test]
fn from_max() {
    assert_eq!(delta_x(&Point::new(MAX, 0.0, 0.0, 0.0), &Point::zero()), MIN + 1.0);
    assert_eq!(delta_x(&Point::new(0.0, MAX, 0.0, 0.0), &Point::zero()), 0.0);
    assert_eq!(delta_x(&Point::new(0.0, 0.0, MAX, 0.0), &Point::zero()), 0.0);
    assert_eq!(delta_x(&Point::new(0.0, 0.0, 0.0, MAX), &Point::zero()), 0.0);
}

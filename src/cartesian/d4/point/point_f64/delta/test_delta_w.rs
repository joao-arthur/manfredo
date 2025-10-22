use super::delta_w;
use crate::cartesian::{
    d1::point::point_f64::{MAX, MIN},
    d4::point::point_f64::Point,
};

#[test]
fn from_min() {
    assert_eq!(delta_w(&Point::of(MIN, 0.0, 0.0, 0.0), &Point::of(-1.0, 0.0, 0.0, 0.0)), 0.0);
    assert_eq!(delta_w(&Point::of(0.0, MIN, 0.0, 0.0), &Point::of(0.0, -1.0, 0.0, 0.0)), 0.0);
    assert_eq!(delta_w(&Point::of(0.0, 0.0, MIN, 0.0), &Point::of(0.0, 0.0, -1.0, 0.0)), 0.0);
    assert_eq!(delta_w(&Point::of(0.0, 0.0, 0.0, MIN), &Point::of(0.0, 0.0, 0.0, -1.0)), MAX);
}

#[test]
fn to_max() {
    assert_eq!(delta_w(&Point::zero(), &Point::of(MAX, 0.0, 0.0, 0.0)), 0.0);
    assert_eq!(delta_w(&Point::zero(), &Point::of(0.0, MAX, 0.0, 0.0)), 0.0);
    assert_eq!(delta_w(&Point::zero(), &Point::of(0.0, 0.0, MAX, 0.0)), 0.0);
    assert_eq!(delta_w(&Point::zero(), &Point::of(0.0, 0.0, 0.0, MAX)), MAX);
}

#[test]
fn to_min() {
    assert_eq!(delta_w(&Point::zero(), &Point::of(MIN, 0.0, 0.0, 0.0)), 0.0);
    assert_eq!(delta_w(&Point::zero(), &Point::of(0.0, MIN, 0.0, 0.0)), 0.0);
    assert_eq!(delta_w(&Point::zero(), &Point::of(0.0, 0.0, MIN, 0.0)), 0.0);
    assert_eq!(delta_w(&Point::zero(), &Point::of(0.0, 0.0, 0.0, MIN)), MIN);
}

#[test]
fn from_max() {
    assert_eq!(delta_w(&Point::of(MAX, 0.0, 0.0, 0.0), &Point::zero()), 0.0);
    assert_eq!(delta_w(&Point::of(0.0, MAX, 0.0, 0.0), &Point::zero()), 0.0);
    assert_eq!(delta_w(&Point::of(0.0, 0.0, MAX, 0.0), &Point::zero()), 0.0);
    assert_eq!(delta_w(&Point::of(0.0, 0.0, 0.0, MAX), &Point::zero()), MIN + 1.0);
}

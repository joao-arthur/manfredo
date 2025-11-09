use super::delta_z;
use crate::cartesian::{
    d1::point::point_f64::{MAX, MIN},
    d3::point::point_f64::Point,
};

#[test]
fn from_min() {
    assert_eq!(delta_z(&Point::new(MIN, 0.0, 0.0), &Point::new(-1.0, 0.0, 0.0)), 0.0);
    assert_eq!(delta_z(&Point::new(0.0, MIN, 0.0), &Point::new(0.0, -1.0, 0.0)), 0.0);
    assert_eq!(delta_z(&Point::new(0.0, 0.0, MIN), &Point::new(0.0, 0.0, -1.0)), MAX);
}

#[test]
fn to_max() {
    assert_eq!(delta_z(&Point::zero(), &Point::new(MAX, 0.0, 0.0)), 0.0);
    assert_eq!(delta_z(&Point::zero(), &Point::new(0.0, MAX, 0.0)), 0.0);
    assert_eq!(delta_z(&Point::zero(), &Point::new(0.0, 0.0, MAX)), MAX);
}

#[test]
fn to_min() {
    assert_eq!(delta_z(&Point::zero(), &Point::new(MIN, 0.0, 0.0)), 0.0);
    assert_eq!(delta_z(&Point::zero(), &Point::new(0.0, MIN, 0.0)), 0.0);
    assert_eq!(delta_z(&Point::zero(), &Point::new(0.0, 0.0, MIN)), MIN);
}

#[test]
fn from_max() {
    assert_eq!(delta_z(&Point::new(MAX, 0.0, 0.0), &Point::zero()), 0.0);
    assert_eq!(delta_z(&Point::new(0.0, MAX, 0.0), &Point::zero()), 0.0);
    assert_eq!(delta_z(&Point::new(0.0, 0.0, MAX), &Point::zero()), MIN + 1.0);
}

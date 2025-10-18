use super::delta_z;
use crate::cartesian::d1::point::point_f64::{MAX, MIN};
use crate::cartesian::d3::point::point_f64::Point;

#[test]
fn from_min() {
    assert_eq!(delta_z(&Point::of(MIN, 0.0, 0.0), &Point::of(-1.0, 0.0, 0.0)), 0.0);
    assert_eq!(delta_z(&Point::of(0.0, MIN, 0.0), &Point::of(0.0, -1.0, 0.0)), 0.0);
    assert_eq!(delta_z(&Point::of(0.0, 0.0, MIN), &Point::of(0.0, 0.0, -1.0)), MAX);
}

#[test]
fn to_max() {
    assert_eq!(delta_z(&Point::of(0.0, 0.0, 0.0), &Point::of(MAX, 0.0, 0.0)), 0.0);
    assert_eq!(delta_z(&Point::of(0.0, 0.0, 0.0), &Point::of(0.0, MAX, 0.0)), 0.0);
    assert_eq!(delta_z(&Point::of(0.0, 0.0, 0.0), &Point::of(0.0, 0.0, MAX)), MAX);
}

#[test]
fn to_min() {
    assert_eq!(delta_z(&Point::of(0.0, 0.0, 0.0), &Point::of(MIN, 0.0, 0.0)), 0.0);
    assert_eq!(delta_z(&Point::of(0.0, 0.0, 0.0), &Point::of(0.0, MIN, 0.0)), 0.0);
    assert_eq!(delta_z(&Point::of(0.0, 0.0, 0.0), &Point::of(0.0, 0.0, MIN)), MIN);
}

#[test]
fn from_max() {
    assert_eq!(delta_z(&Point::of(MAX, 0.0, 0.0), &Point::of(0.0, 0.0, 0.0)), 0.0);
    assert_eq!(delta_z(&Point::of(0.0, MAX, 0.0), &Point::of(0.0, 0.0, 0.0)), 0.0);
    assert_eq!(delta_z(&Point::of(0.0, 0.0, MAX), &Point::of(0.0, 0.0, 0.0)), MIN + 1.0);
}

use super::delta_min;
use crate::cartesian::d2::point::point_f32::{MAX, Point};

#[test]
fn test_delta_min() {
    assert_eq!(delta_min(&Point::of(0.0, 5.0), &Point::of(10.0, 10.0)), 5.0);
    assert_eq!(delta_min(&Point::of(5.0, 0.0), &Point::of(9.0, 9.0)), 4.0);
}

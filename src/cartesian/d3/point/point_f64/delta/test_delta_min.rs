use super::delta_min;
use crate::cartesian::d3::point::point_f64::Point;

#[test]
fn test_delta_min() {
    assert_eq!(delta_min(&Point::of(0.0, 1.0, 2.0), &Point::of(10.0, 10.0, 10.0)), 8.0);
    assert_eq!(delta_min(&Point::of(1.0, 2.0, 0.0), &Point::of(9.0, 9.0, 9.0)), 7.0);
    assert_eq!(delta_min(&Point::of(2.0, 0.0, 1.0), &Point::of(8.0, 8.0, 8.0)), 6.0);
}

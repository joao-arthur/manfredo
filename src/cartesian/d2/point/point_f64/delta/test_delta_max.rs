use super::delta_max;
use crate::cartesian::d2::point::point_f64::Point;

#[test]
fn test_delta_max() {
    assert_eq!(delta_max(&Point::of(0.0, 5.0), &Point::of(10.0, 10.0)), 10.0);
    assert_eq!(delta_max(&Point::of(5.0, 0.0), &Point::of(9.0, 9.0)), 9.0);
}

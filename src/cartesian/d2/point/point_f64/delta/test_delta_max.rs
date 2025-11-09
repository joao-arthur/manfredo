use super::delta_max;
use crate::cartesian::d2::point::point_f64::Point;

#[test]
fn test_delta_max() {
    assert_eq!(delta_max(&Point::new(0.0, 5.0), &Point::new(10.0, 10.0)), 10.0);
    assert_eq!(delta_max(&Point::new(5.0, 0.0), &Point::new(9.0, 9.0)), 9.0);
}

use super::delta_max;
use crate::cartesian::d4::point::point_f64::Point;

#[test]
fn test_delta_max() {
    assert_eq!(delta_max(&Point::of(0.0, 1.0, 2.0, 3.0), &Point::of(10.0, 10.0, 10.0, 10.0)), 10.0);
    assert_eq!(delta_max(&Point::of(1.0, 2.0, 3.0, 0.0), &Point::of(9.0, 9.0, 9.0, 9.0)), 9.0);
    assert_eq!(delta_max(&Point::of(2.0, 3.0, 0.0, 1.0), &Point::of(8.0, 8.0, 8.0, 8.0)), 8.0);
    assert_eq!(delta_max(&Point::of(3.0, 0.0, 1.0, 2.0), &Point::of(7.0, 7.0, 7.0, 7.0)), 7.0);
}

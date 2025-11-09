use super::delta_max;
use crate::cartesian::d3::point::point_f32::Point;

#[test]
fn test_delta_max() {
    assert_eq!(delta_max(&Point::new(0.0, 1.0, 2.0), &Point::new(10.0, 10.0, 10.0)), 10.0);
    assert_eq!(delta_max(&Point::new(1.0, 2.0, 0.0), &Point::new(9.0, 9.0, 9.0)), 9.0);
    assert_eq!(delta_max(&Point::new(2.0, 0.0, 1.0), &Point::new(8.0, 8.0, 8.0)), 8.0);
}

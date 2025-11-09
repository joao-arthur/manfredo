use super::delta_y;
use crate::cartesian::d2::point::point_f64::{MAX, Point};

#[test]
fn test_delta_y() {
    assert_eq!(delta_y(&Point::zero(), &Point::new(MAX, 0.0)), 0.0);
    assert_eq!(delta_y(&Point::zero(), &Point::new(0.0, MAX)), MAX);
    assert_eq!(delta_y(&Point::new(-4_503_599_627_370_496.0, 0.0), &Point::new(4_503_599_627_370_495.0, 0.0)), 0.0);
    assert_eq!(delta_y(&Point::new(0.0, -4_503_599_627_370_496.0), &Point::new(0.0, 4_503_599_627_370_495.0)), MAX);
}

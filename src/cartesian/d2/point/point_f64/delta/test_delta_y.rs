use super::delta_y;
use crate::cartesian::d2::point::point_f64::{MAX, Point};

#[test]
fn test_delta_y() {
    assert_eq!(delta_y(&Point::of(0.0, 0.0), &Point::of(MAX, 0.0)), 0.0);
    assert_eq!(delta_y(&Point::of(0.0, 0.0), &Point::of(0.0, MAX)), MAX);
    assert_eq!(delta_y(&Point::of(-4_503_599_627_370_496.0, 0.0), &Point::of(4_503_599_627_370_495.0, 0.0)), 0.0);
    assert_eq!(delta_y(&Point::of(0.0, -4_503_599_627_370_496.0), &Point::of(0.0, 4_503_599_627_370_495.0)), MAX);
}

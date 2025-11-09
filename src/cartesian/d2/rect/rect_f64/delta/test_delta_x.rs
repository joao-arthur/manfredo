use super::delta_x;
use crate::cartesian::{d1::point::point_f64::MAX, d2::rect::rect_f64::Rect};

#[test]
fn test_delta_x() {
    assert_eq!(delta_x(&Rect::new((0.0, 0.0), (0.0, MAX))), 0.0);
    assert_eq!(delta_x(&Rect::new((0.0, 0.0), (MAX, 0.0))), MAX);
    assert_eq!(delta_x(&Rect::new((0.0, -4_503_599_627_370_496.0), (0.0, 4_503_599_627_370_495.0))), 0.0);
    assert_eq!(delta_x(&Rect::new((-4_503_599_627_370_496.0, 0.0), (4_503_599_627_370_495.0, 0.0))), MAX);
}

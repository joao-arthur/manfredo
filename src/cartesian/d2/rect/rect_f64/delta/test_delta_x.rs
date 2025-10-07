use super::delta_x;
use crate::cartesian::d2::{point::point_f64::MAX, rect::rect_f64::Rect};

#[test]
fn test_delta_x() {
    assert_eq!(delta_x(&Rect::of(0.0, 0.0, 0.0, MAX)), 0.0);
    assert_eq!(delta_x(&Rect::of(0.0, 0.0, MAX, 0.0)), MAX);
    assert_eq!(delta_x(&Rect::of(0.0, -4_503_599_627_370_496.0, 0.0, 4_503_599_627_370_495.0)), 0.0);
    assert_eq!(delta_x(&Rect::of(-4_503_599_627_370_496.0, 0.0, 4_503_599_627_370_495.0, 0.0)), MAX);
}

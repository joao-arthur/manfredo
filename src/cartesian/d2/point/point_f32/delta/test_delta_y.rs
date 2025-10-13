use super::delta_y;
use crate::cartesian::d2::point::point_f32::{MAX, Point};

#[test]
fn test_delta_y() {
    assert_eq!(delta_y(&Point::of(0.0, 0.0), &Point::of(MAX, 0.0)), 0.0);
    assert_eq!(delta_y(&Point::of(0.0, 0.0), &Point::of(0.0, MAX)), MAX);
    assert_eq!(delta_y(&Point::of(-8_388_608.0, 0.0), &Point::of(8_388_607.0, 0.0)), 0.0);
    assert_eq!(delta_y(&Point::of(0.0, -8_388_608.0), &Point::of(0.0, 8_388_607.0)), MAX);
}

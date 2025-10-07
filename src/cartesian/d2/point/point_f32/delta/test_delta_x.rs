use super::delta_x;
use crate::cartesian::d2::point::point_f32::{MAX, Point};

#[test]
fn test_delta_x() {
    assert_eq!(delta_x(&Point::of(0.0, 0.0), &Point::of(0.0, MAX)), 0.0);
    assert_eq!(delta_x(&Point::of(0.0, -8_388_608.0), &Point::of(0.0, 8_388_607.0)), 0.0);
    assert_eq!(delta_x(&Point::of(0.0, 0.0), &Point::of(MAX, 0.0)), MAX);
    assert_eq!(delta_x(&Point::of(-8_388_608.0, 0.0), &Point::of(8_388_607.0, 0.0)), MAX);
}

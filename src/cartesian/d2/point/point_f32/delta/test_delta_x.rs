use super::delta_x;
use crate::cartesian::d2::point::point_f32::{MAX, Point};

#[test]
fn test_delta_x() {
    assert_eq!(delta_x(&Point::zero(), &Point::new(0.0, MAX)), 0.0);
    assert_eq!(delta_x(&Point::zero(), &Point::new(MAX, 0.0)), MAX);
    assert_eq!(delta_x(&Point::new(0.0, -8_388_608.0), &Point::new(0.0, 8_388_607.0)), 0.0);
    assert_eq!(delta_x(&Point::new(-8_388_608.0, 0.0), &Point::new(8_388_607.0, 0.0)), MAX);
}

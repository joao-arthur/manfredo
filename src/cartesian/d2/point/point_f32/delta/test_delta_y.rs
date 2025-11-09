use super::delta_y;
use crate::cartesian::d2::point::point_f32::{MAX, Point};

#[test]
fn test_delta_y() {
    assert_eq!(delta_y(&Point::zero(), &Point::new(MAX, 0.0)), 0.0);
    assert_eq!(delta_y(&Point::zero(), &Point::new(0.0, MAX)), MAX);
    assert_eq!(delta_y(&Point::new(-8_388_608.0, 0.0), &Point::new(8_388_607.0, 0.0)), 0.0);
    assert_eq!(delta_y(&Point::new(0.0, -8_388_608.0), &Point::new(0.0, 8_388_607.0)), MAX);
}

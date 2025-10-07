use super::delta_max;
use crate::cartesian::d2::point::point_u16::Point;

const MAX: u16 = u16::MAX;

#[test]
fn test_delta_max() {
    assert_eq!(delta_max(&Point::of(0, 5), &Point::of(10, 10)), 10);
    assert_eq!(delta_max(&Point::of(5, 0), &Point::of(9, 9)), 9);
}

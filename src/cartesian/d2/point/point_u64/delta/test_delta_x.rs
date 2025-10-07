use super::{delta, delta_max, delta_min, delta_x, delta_y};
use crate::cartesian::d2::point::point_u64::Point;

const MAX: u64 = u64::MAX;

#[test]
fn test_delta_x() {
    assert_eq!(delta_x(&Point::min(), &Point::of(0, MAX)), 0);
    assert_eq!(delta_x(&Point::min(), &Point::of(MAX, 0)), MAX);
}

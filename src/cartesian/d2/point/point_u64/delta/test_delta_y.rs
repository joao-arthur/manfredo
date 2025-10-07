use super::{delta, delta_max, delta_min, delta_x, delta_y};
use crate::cartesian::d2::point::point_u64::Point;

const MAX: u64 = u64::MAX;

#[test]
fn test_delta_y() {
    assert_eq!(delta_y(&Point::min(), &Point::of(MAX, 0)), 0);
    assert_eq!(delta_y(&Point::min(), &Point::of(0, MAX)), MAX);
}

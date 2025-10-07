use super::delta_y;
use crate::cartesian::d2::point::point_i64::Point;

const MIN: i64 = i64::MIN;
const MAX: i64 = i64::MAX;

#[test]
fn test_delta_y() {
    assert_eq!(delta_y(&Point::of(MIN, 0), &Point::of(MAX, 0)), 0);
    assert_eq!(delta_y(&Point::of(0, MIN), &Point::of(0, MAX)), u64::MAX);
}

use super::delta_x;
use crate::cartesian::d2::point::{point_i64::Point, point_u64};

const MIN: i64 = i64::MIN;
const MAX: i64 = i64::MAX;

#[test]
fn test_delta_x() {
    assert_eq!(delta_x(&Point::of(0, MIN), &Point::of(0, MAX)), 0);
    assert_eq!(delta_x(&Point::of(MIN, 0), &Point::of(MAX, 0)), u64::MAX);
}

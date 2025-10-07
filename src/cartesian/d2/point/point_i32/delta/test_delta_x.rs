use super::delta_x;
use crate::cartesian::d2::point::point_i32::Point;

const MIN: i32 = i32::MIN;
const MAX: i32 = i32::MAX;

#[test]
fn test_delta_x() {
    assert_eq!(delta_x(&Point::of(0, MIN), &Point::of(0, MAX)), 0);
    assert_eq!(delta_x(&Point::of(MIN, 0), &Point::of(MAX, 0)), u32::MAX);
}

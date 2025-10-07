use super::delta_y;
use crate::cartesian::d2::point::{point_i32::Point, point_u32};

const MIN: i32 = i32::MIN;
const MAX: i32 = i32::MAX;

#[test]
fn test_delta_y() {
    assert_eq!(delta_y(&Point::of(MIN, 0), &Point::of(MAX, 0)), 0);
    assert_eq!(delta_y(&Point::of(0, MIN), &Point::of(0, MAX)), u32::MAX);
}

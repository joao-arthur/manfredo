use super::delta_row;
use crate::matrix::d2::point::point_i32::Point;

const MIN: i32 = i32::MIN;
const MAX: i32 = i32::MAX;

#[test]
fn test_delta_row() {
    assert_eq!(delta_row(&Point::of(0, MIN), &Point::of(0, MAX)), 0);
    assert_eq!(delta_row(&Point::of(MIN, 0), &Point::of(MAX, 0)), u32::MAX);
}

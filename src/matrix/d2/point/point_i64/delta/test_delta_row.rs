use super::delta_row;
use crate::matrix::d2::point::point_i64::Point;

const MIN: i64 = i64::MIN;
const MAX: i64 = i64::MAX;

#[test]
fn test_delta_row() {
    assert_eq!(delta_row(&Point::of(0, MIN), &Point::of(0, MAX)), 0);
    assert_eq!(delta_row(&Point::of(MIN, 0), &Point::of(MAX, 0)), u64::MAX);
}

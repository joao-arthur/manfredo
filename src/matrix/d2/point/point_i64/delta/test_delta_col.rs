use super::delta_col;
use crate::matrix::d2::point::{point_i64::Point, point_u64};

const MIN: i64 = i64::MIN;
const MAX: i64 = i64::MAX;

#[test]
fn test_delta_col() {
    assert_eq!(delta_col(&Point::of(MIN, 0), &Point::of(MAX, 0)), 0);
    assert_eq!(delta_col(&Point::of(0, MIN), &Point::of(0, MAX)), u64::MAX);
}

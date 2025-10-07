use super::delta_max;
use crate::matrix::d2::point::{point_i64::Point, point_u64};

const MIN: i64 = i64::MIN;
const MAX: i64 = i64::MAX;

#[test]
fn test_delta_max() {
    assert_eq!(delta_max(&Point::of(0, 5), &Point::of(10, 10)), 10);
    assert_eq!(delta_max(&Point::of(5, 0), &Point::of(9, 9)), 9);
}

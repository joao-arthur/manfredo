use super::delta_min;
use crate::cartesian::d2::point::{point_i32::Point, point_u32};

const MIN: i32 = i32::MIN;
const MAX: i32 = i32::MAX;

#[test]
fn test_delta_min() {
    assert_eq!(delta_min(&Point::of(0, 5), &Point::of(10, 10)), 5);
    assert_eq!(delta_min(&Point::of(5, 0), &Point::of(9, 9)), 4);
}

use super::delta_min;
use crate::cartesian::d2::point::{point_i8::Point, point_u8};

const MIN: i8 = i8::MIN;
const MAX: i8 = i8::MAX;

#[test]
fn test_delta_min() {
    assert_eq!(delta_min(&Point::of(0, 5), &Point::of(10, 10)), 5);
    assert_eq!(delta_min(&Point::of(5, 0), &Point::of(9, 9)), 4);
}

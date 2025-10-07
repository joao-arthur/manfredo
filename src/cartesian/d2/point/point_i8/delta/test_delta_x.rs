use super::delta_x;
use crate::cartesian::d2::point::{point_i8::Point, point_u8};

const MIN: i8 = i8::MIN;
const MAX: i8 = i8::MAX;

#[test]
fn test_delta_x() {
    assert_eq!(delta_x(&Point::of(0, MIN), &Point::of(0, MAX)), 0);
    assert_eq!(delta_x(&Point::of(MIN, 0), &Point::of(MAX, 0)), u8::MAX);
}

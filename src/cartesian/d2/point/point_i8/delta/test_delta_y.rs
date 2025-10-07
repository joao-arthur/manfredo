use super::delta_y;
use crate::cartesian::d2::point::{point_i8::Point, point_u8};

const MIN: i8 = i8::MIN;
const MAX: i8 = i8::MAX;

#[test]
fn test_delta_y() {
    assert_eq!(delta_y(&Point::of(MIN, 0), &Point::of(MAX, 0)), 0);
    assert_eq!(delta_y(&Point::of(0, MIN), &Point::of(0, MAX)), u8::MAX);
}

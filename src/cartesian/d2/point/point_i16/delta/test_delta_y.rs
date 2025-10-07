use super::delta_y;
use crate::cartesian::d2::point::{point_i16::Point, point_u16};

const MIN: i16 = i16::MIN;
const MAX: i16 = i16::MAX;

#[test]
fn test_delta_y() {
    assert_eq!(delta_y(&Point::of(MIN, 0), &Point::of(MAX, 0)), 0);
    assert_eq!(delta_y(&Point::of(0, MIN), &Point::of(0, MAX)), u16::MAX);
}

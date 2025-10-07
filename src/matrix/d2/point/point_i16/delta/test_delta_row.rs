use super::delta_row;
use crate::matrix::d2::point::{point_i16::Point, point_u16};

const MIN: i16 = i16::MIN;
const MAX: i16 = i16::MAX;

#[test]
fn test_delta_row() {
    assert_eq!(delta_row(&Point::of(0, MIN), &Point::of(0, MAX)), 0);
    assert_eq!(delta_row(&Point::of(MIN, 0), &Point::of(MAX, 0)), u16::MAX);
}

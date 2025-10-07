use super::delta_col;
use crate::matrix::d2::point::{point_i16::Point, point_u16};

const MIN: i16 = i16::MIN;
const MAX: i16 = i16::MAX;


#[test]
fn test_delta_col() {
    assert_eq!(delta_col(&Point::of(MIN, 0), &Point::of(MAX, 0)), 0);
    assert_eq!(delta_col(&Point::of(0, MIN), &Point::of(0, MAX)), u16::MAX);
}

use super::delta_min;
use crate::matrix::d2::point::{point_i16::Point, point_u16};

const MIN: i16 = i16::MIN;
const MAX: i16 = i16::MAX;

#[test]
fn test_delta_min() {
    assert_eq!(delta_min(&Point::of(0, 5), &Point::of(10, 10)), 5);
    assert_eq!(delta_min(&Point::of(5, 0), &Point::of(9, 9)), 4);
}

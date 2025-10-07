use super::delta_row;
use crate::matrix::d2::point::point_u8::Point;

const MAX: u8 = u8::MAX;

#[test]
fn test_delta_row() {
    assert_eq!(delta_row(&Point::min(), &Point::of(0, MAX)), 0);
    assert_eq!(delta_row(&Point::min(), &Point::of(MAX, 0)), MAX);
}

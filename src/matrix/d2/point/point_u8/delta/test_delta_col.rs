use super::delta_col;
use crate::matrix::d2::point::point_u8::Point;

const MAX: u8 = u8::MAX;

#[test]
fn test_delta_col() {
    assert_eq!(delta_col(&Point::min(), &Point::of(MAX, 0)), 0);
    assert_eq!(delta_col(&Point::min(), &Point::of(0, MAX)), MAX);
}

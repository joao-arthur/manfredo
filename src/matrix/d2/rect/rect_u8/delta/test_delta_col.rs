use super::delta_col;
use crate::matrix::{d1::point::point_u8::MAX, d2::rect::rect_u8::Rect};

#[test]
fn test_delta_col() {
    assert_eq!(delta_col(&Rect::of(0, 0, 0, 0)), 0);
    assert_eq!(delta_col(&Rect::of(0, 0, 0, MAX)), MAX);
}

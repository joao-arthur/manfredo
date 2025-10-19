use super::delta_col;
use crate::matrix::{
    d1::point::point_i16::{MAX, MIN},
    d2::rect::rect_i16::Rect,
};

#[test]
fn test_delta_col() {
    assert_eq!(delta_col(&Rect::of(MIN, 0, MAX, 0)), 0);
    assert_eq!(delta_col(&Rect::of(0, MIN, 0, MAX)), u16::MAX);
}

use super::delta_row;
use crate::matrix::{
    d1::point::point_i16::{MAX, MIN},
    d2::rect::rect_i16::Rect,
};

#[test]
fn test_delta_row() {
    assert_eq!(delta_row(&Rect::of(0, MIN, 0, MAX)), 0);
    assert_eq!(delta_row(&Rect::of(MIN, 0, MAX, 0)), u16::MAX);
}

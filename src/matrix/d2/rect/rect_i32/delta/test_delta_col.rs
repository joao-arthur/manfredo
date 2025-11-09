use super::delta_col;
use crate::matrix::{
    d1::point::point_i32::{MAX, MIN},
    d2::rect::rect_i32::Rect,
};

#[test]
fn test_delta_col() {
    assert_eq!(delta_col(&Rect::new((MIN, 0), (MAX, 0))), 0);
    assert_eq!(delta_col(&Rect::new((0, MIN), (0, MAX))), u32::MAX);
}

use super::delta_row;
use crate::matrix::{
    d1::point::point_i32::{MAX, MIN},
    d2::rect::rect_i32::Rect,
};

#[test]
fn test_delta_row() {
    assert_eq!(delta_row(&Rect::new((0, MIN), (0, MAX))), 0);
    assert_eq!(delta_row(&Rect::new((MIN, 0), (MAX, 0))), u32::MAX);
}

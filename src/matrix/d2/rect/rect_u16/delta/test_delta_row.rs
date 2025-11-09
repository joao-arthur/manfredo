use super::delta_row;
use crate::matrix::{d1::point::point_u16::MAX, d2::rect::rect_u16::Rect};

#[test]
fn test_delta_row() {
    assert_eq!(delta_row(&Rect::min()), 0);
    assert_eq!(delta_row(&Rect::new((0, 0), (MAX, 0))), MAX);
}

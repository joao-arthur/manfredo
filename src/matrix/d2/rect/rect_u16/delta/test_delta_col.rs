use super::delta_col;
use crate::matrix::d2::rect::rect_u16::Rect;

const MAX: u16 = u16::MAX;

#[test]
fn test_delta_col() {
    assert_eq!(delta_col(&Rect::of(0, 0, 0, 0)), 0);
    assert_eq!(delta_col(&Rect::of(0, 0, 0, MAX)), MAX);
}

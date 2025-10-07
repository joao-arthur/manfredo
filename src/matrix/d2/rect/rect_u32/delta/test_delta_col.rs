use super::delta_col;
use crate::matrix::d2::rect::rect_u32::Rect;

const MAX: u32 = u32::MAX;

#[test]
fn test_delta_col() {
    assert_eq!(delta_col(&Rect::of(0, 0, 0, 0)), 0);
    assert_eq!(delta_col(&Rect::of(0, 0, 0, MAX)), MAX);
}

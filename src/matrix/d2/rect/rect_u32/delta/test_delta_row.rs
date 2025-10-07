use super::delta_row;
use crate::matrix::d2::rect::rect_u32::Rect;

const MAX: u32 = u32::MAX;

#[test]
fn test_delta_row() {
    assert_eq!(delta_row(&Rect::of(0, 0, 0, 0)), 0);
    assert_eq!(delta_row(&Rect::of(0, 0, MAX, 0)), MAX);
}

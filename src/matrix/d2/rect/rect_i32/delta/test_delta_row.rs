use super::delta_row;
use crate::matrix::d2::rect::rect_i32::Rect;

const MIN: i32 = i32::MIN;
const MAX: i32 = i32::MAX;

#[test]
fn test_delta_row() {
    assert_eq!(delta_row(&Rect::of(0, MIN, 0, MAX)), 0);
    assert_eq!(delta_row(&Rect::of(MIN, 0, MAX, 0)), u32::MAX);
}

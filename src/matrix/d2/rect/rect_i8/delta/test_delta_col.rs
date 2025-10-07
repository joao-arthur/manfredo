use super::delta_col;
use crate::matrix::d2::rect::rect_i8::Rect;

const MIN: i8 = i8::MIN;
const MAX: i8 = i8::MAX;

#[test]
fn test_delta_col() {
    assert_eq!(delta_col(&Rect::of(MIN, 0, MAX, 0)), 0);
    assert_eq!(delta_col(&Rect::of(0, MIN, 0, MAX)), u8::MAX);
}

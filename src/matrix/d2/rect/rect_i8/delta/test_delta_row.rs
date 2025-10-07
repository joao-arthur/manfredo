use super::delta_row;
use crate::matrix::d2::rect::rect_i8::Rect;

const MIN: i8 = i8::MIN;
const MAX: i8 = i8::MAX;

#[test]
fn test_delta_row() {
    assert_eq!(delta_row(&Rect::of(0, MIN, 0, MAX)), 0);
    assert_eq!(delta_row(&Rect::of(MIN, 0, MAX, 0)), u8::MAX);
}

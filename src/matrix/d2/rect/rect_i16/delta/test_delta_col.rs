use super::delta_col;
use crate::matrix::d2::rect::rect_i16::Rect;

const MIN: i16 = i16::MIN;
const MAX: i16 = i16::MAX;

#[test]
fn test_delta_col() {
    assert_eq!(delta_col(&Rect::of(MIN, 0, MAX, 0)), 0);
    assert_eq!(delta_col(&Rect::of(0, MIN, 0, MAX)), u16::MAX);
}

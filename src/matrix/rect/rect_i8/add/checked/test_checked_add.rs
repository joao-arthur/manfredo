use super::checked_add;
use crate::matrix::rect::rect_i8::RectI8;

#[test]
fn test_checked_add() {
    assert_eq!(checked_add(&RectI8::of(-7, 9, -12, 15), &RectI8::of(5, 4, 3, 2)), RectI8::of(-2, 13, -9, 17));
    assert_eq!(checked_add(&RectI8::of(5, 4, 15, 17), &RectI8::of(-14, -13, -12, -11)), RectI8::of(-9, -9, 3, 6));
}

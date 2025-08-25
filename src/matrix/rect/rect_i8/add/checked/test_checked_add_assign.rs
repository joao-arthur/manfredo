
use super::checked_add_assign;
use crate::matrix::rect::rect_i8::RectI8;

#[test]
fn test_checked_add_assign() {
    let mut r = RectI8::of(0, 0, 12, 15);
    checked_add_assign(&mut r, &RectI8::of(5, 4, 3, 2));
    assert_eq!(r, RectI8::of(5, 4, 15, 17));
    checked_add_assign(&mut r, &RectI8::of(-14, -13, -12, -11));
    assert_eq!(r, RectI8::of(-9, -9, 3, 6));
}

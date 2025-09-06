use super::checked_add_assign;
use crate::matrix::rect::rect_i16::RectI16;

#[test]
fn test() {
    let mut r = RectI16::of(-7, 9, -12, 15);
    checked_add_assign(&mut r, &RectI16::of(5, 4, 3, 2));
    assert_eq!(r, RectI16::of(-2, 13, -9, 17));
    checked_add_assign(&mut r, &RectI16::of(9, -10, 11, -12));
    assert_eq!(r, RectI16::of(7, 3, 2, 5));
}

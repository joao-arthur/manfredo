use super::checked_add_assign;
use crate::matrix::rect::{rect_i16::RectI16, rect_u16::RectU16};

#[test]
fn test() {
    let mut r = RectU16::of(0, 0, 12, 10);
    checked_add_assign(&mut r, &RectI16::of(5, 4, 3, 2));
    assert_eq!(r, RectU16::of(5, 4, 15, 12));
    checked_add_assign(&mut r, &RectI16::of(-4, -3, -2, -1));
    assert_eq!(r, RectU16::of(1, 1, 13, 11));
}

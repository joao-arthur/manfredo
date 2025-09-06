use super::checked_inflate_assign;
use crate::cartesian::rect::rect_u16::RectU16;

#[test]
fn min_bounds() {
    let mut r = RectU16::of(7, 3, 9, 13);
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectU16::of(6, 2, 10, 14));
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectU16::of(5, 1, 11, 15));
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectU16::of(4, 0, 12, 16));
}

#[test]
fn max_bounds() {
    let mut r = RectU16::of(u16::MAX - 33, u16::MAX - 17, u16::MAX - 5, u16::MAX - 3);
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectU16::of(u16::MAX - 34, u16::MAX - 18, u16::MAX - 4, u16::MAX - 2));
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectU16::of(u16::MAX - 35, u16::MAX - 19, u16::MAX - 3, u16::MAX - 1));
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectU16::of(u16::MAX - 36, u16::MAX - 20, u16::MAX - 2, u16::MAX));
}

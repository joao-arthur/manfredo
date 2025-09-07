use super::checked_inflate_assign;
use crate::cartesian::rect::rect_u8::RectU8;

const MAX: u8 = u8::MAX;

#[test]
fn min_bounds() {
    let mut r = RectU8::of(7, 3, 9, 13);
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectU8::of(6, 2, 10, 14));
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectU8::of(5, 1, 11, 15));
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectU8::of(4, 0, 12, 16));
}

#[test]
fn max_bounds() {
    let mut r = RectU8::of(MAX - 33, MAX - 17, MAX - 5, MAX - 3);
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectU8::of(MAX - 34, MAX - 18, MAX - 4, MAX - 2));
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectU8::of(MAX - 35, MAX - 19, MAX - 3, MAX - 1));
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectU8::of(MAX - 36, MAX - 20, MAX - 2, MAX));
}

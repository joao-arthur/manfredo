use super::saturating_inflate_assign;
use crate::cartesian::rect::rect_u16::RectU16;

const MAX: u16 = u16::MAX;

#[test]
fn min_bounds() {
    let mut r = RectU16::of(7, 2, 17, 13);
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU16::of(6, 1, 18, 14));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU16::of(5, 0, 19, 15));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU16::of(4, 0, 20, 17));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU16::of(3, 0, 21, 19));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU16::of(2, 0, 22, 21));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU16::of(1, 0, 23, 23));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU16::of(0, 0, 24, 25));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU16::of(0, 0, 26, 27));
}

#[test]
fn max_bounds() {
    let mut r = RectU16::of(MAX - 33, MAX - 17, MAX - 5, MAX - 3);
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU16::of(MAX - 34, MAX - 18, MAX - 4, MAX - 2));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU16::of(MAX - 35, MAX - 19, MAX - 3, MAX - 1));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU16::of(MAX - 36, MAX - 20, MAX - 2, MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU16::of(MAX - 37, MAX - 22, MAX - 1, MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU16::of(MAX - 38, MAX - 24, MAX, MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU16::of(MAX - 40, MAX - 26, MAX, MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU16::of(MAX - 42, MAX - 28, MAX, MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU16::of(MAX - 44, MAX - 30, MAX, MAX));
}

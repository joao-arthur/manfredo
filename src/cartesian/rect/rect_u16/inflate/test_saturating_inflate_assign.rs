use super::saturating_inflate_assign;
use crate::cartesian::rect::rect_u16::RectU16;

#[test]
fn saturating_inflate_assign_min_bounds() {
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
fn saturating_inflate_assign_max_bounds() {
    let mut r = RectU16::of(u16::MAX - 33, u16::MAX - 17, u16::MAX - 5, u16::MAX - 3);
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU16::of(u16::MAX - 34, u16::MAX - 18, u16::MAX - 4, u16::MAX - 2));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU16::of(u16::MAX - 35, u16::MAX - 19, u16::MAX - 3, u16::MAX - 1));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU16::of(u16::MAX - 36, u16::MAX - 20, u16::MAX - 2, u16::MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU16::of(u16::MAX - 37, u16::MAX - 22, u16::MAX - 1, u16::MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU16::of(u16::MAX - 38, u16::MAX - 24, u16::MAX, u16::MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU16::of(u16::MAX - 40, u16::MAX - 26, u16::MAX, u16::MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU16::of(u16::MAX - 42, u16::MAX - 28, u16::MAX, u16::MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU16::of(u16::MAX - 44, u16::MAX - 30, u16::MAX, u16::MAX));
}

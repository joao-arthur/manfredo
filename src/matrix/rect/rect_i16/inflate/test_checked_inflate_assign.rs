use super::checked_inflate_assign;
use crate::matrix::rect::rect_i16::RectI16;

#[test]
fn min_bounds() {
    let mut r = RectI16::of(i16::MIN + 7, i16::MIN + 3, i16::MIN + 9, i16::MIN + 13);
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectI16::of(i16::MIN + 6, i16::MIN + 2, i16::MIN + 10, i16::MIN + 14));
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectI16::of(i16::MIN + 5, i16::MIN + 1, i16::MIN + 11, i16::MIN + 15));
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectI16::of(i16::MIN + 4, i16::MIN, i16::MIN + 12, i16::MIN + 16));
}

#[test]
fn max_bounds() {
    let mut r = RectI16::of(i16::MAX - 33, i16::MAX - 17, i16::MAX - 5, i16::MAX - 3);
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectI16::of(i16::MAX - 34, i16::MAX - 18, i16::MAX - 4, i16::MAX - 2));
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectI16::of(i16::MAX - 35, i16::MAX - 19, i16::MAX - 3, i16::MAX - 1));
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectI16::of(i16::MAX - 36, i16::MAX - 20, i16::MAX - 2, i16::MAX));
}

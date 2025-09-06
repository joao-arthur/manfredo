use super::saturating_inflate_assign;
use crate::matrix::rect::rect_i8::RectI8;

#[test]
fn min_bounds() {
    let mut r = RectI8::of(i8::MIN + 7, i8::MIN + 2, i8::MIN + 17, i8::MIN + 13);
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI8::of(i8::MIN + 6, i8::MIN + 1, i8::MIN + 18, i8::MIN + 14));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI8::of(i8::MIN + 5, i8::MIN, i8::MIN + 19, i8::MIN + 15));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI8::of(i8::MIN + 4, i8::MIN, i8::MIN + 20, i8::MIN + 17));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI8::of(i8::MIN + 3, i8::MIN, i8::MIN + 21, i8::MIN + 19));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI8::of(i8::MIN + 2, i8::MIN, i8::MIN + 22, i8::MIN + 21));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI8::of(i8::MIN + 1, i8::MIN, i8::MIN + 23, i8::MIN + 23));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI8::of(i8::MIN, i8::MIN, i8::MIN + 24, i8::MIN + 25));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI8::of(i8::MIN, i8::MIN, i8::MIN + 26, i8::MIN + 27));
}

#[test]
fn max_bounds() {
    let mut r = RectI8::of(i8::MAX - 33, i8::MAX - 17, i8::MAX - 5, i8::MAX - 3);
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI8::of(i8::MAX - 34, i8::MAX - 18, i8::MAX - 4, i8::MAX - 2));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI8::of(i8::MAX - 35, i8::MAX - 19, i8::MAX - 3, i8::MAX - 1));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI8::of(i8::MAX - 36, i8::MAX - 20, i8::MAX - 2, i8::MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI8::of(i8::MAX - 37, i8::MAX - 22, i8::MAX - 1, i8::MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI8::of(i8::MAX - 38, i8::MAX - 24, i8::MAX, i8::MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI8::of(i8::MAX - 40, i8::MAX - 26, i8::MAX, i8::MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI8::of(i8::MAX - 42, i8::MAX - 28, i8::MAX, i8::MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI8::of(i8::MAX - 44, i8::MAX - 30, i8::MAX, i8::MAX));
}

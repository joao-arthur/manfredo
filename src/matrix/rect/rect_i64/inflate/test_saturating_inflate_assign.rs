use super::saturating_inflate_assign;
use crate::matrix::rect::rect_i64::RectI64;

#[test]
fn min_bounds() {
    let mut r = RectI64::of(i64::MIN + 7, i64::MIN + 2, i64::MIN + 17, i64::MIN + 13);
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MIN + 6, i64::MIN + 1, i64::MIN + 18, i64::MIN + 14));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MIN + 5, i64::MIN, i64::MIN + 19, i64::MIN + 15));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MIN + 4, i64::MIN, i64::MIN + 20, i64::MIN + 17));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MIN + 3, i64::MIN, i64::MIN + 21, i64::MIN + 19));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MIN + 2, i64::MIN, i64::MIN + 22, i64::MIN + 21));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MIN + 1, i64::MIN, i64::MIN + 23, i64::MIN + 23));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 24, i64::MIN + 25));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 26, i64::MIN + 27));
}

#[test]
fn max_bounds() {
    let mut r = RectI64::of(i64::MAX - 33, i64::MAX - 17, i64::MAX - 5, i64::MAX - 3);
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MAX - 36, i64::MAX - 20, i64::MAX - 2, i64::MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MAX - 37, i64::MAX - 22, i64::MAX - 1, i64::MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MAX - 38, i64::MAX - 24, i64::MAX, i64::MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MAX - 40, i64::MAX - 26, i64::MAX, i64::MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MAX - 42, i64::MAX - 28, i64::MAX, i64::MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MAX - 44, i64::MAX - 30, i64::MAX, i64::MAX));
}

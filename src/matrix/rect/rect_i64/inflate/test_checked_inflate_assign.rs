use super::checked_inflate_assign;
use crate::matrix::rect::rect_i64::RectI64;

#[test]
fn min_bounds() {
    let mut r = RectI64::of(i64::MIN + 7, i64::MIN + 3, i64::MIN + 9, i64::MIN + 13);
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MIN + 6, i64::MIN + 2, i64::MIN + 10, i64::MIN + 14));
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MIN + 5, i64::MIN + 1, i64::MIN + 11, i64::MIN + 15));
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MIN + 4, i64::MIN, i64::MIN + 12, i64::MIN + 16));
}

#[test]
fn max_bounds() {
    let mut r = RectI64::of(i64::MAX - 33, i64::MAX - 17, i64::MAX - 5, i64::MAX - 3);
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2));
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1));
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MAX - 36, i64::MAX - 20, i64::MAX - 2, i64::MAX));
}

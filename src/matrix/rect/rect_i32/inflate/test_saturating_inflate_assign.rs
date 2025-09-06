use super::saturating_inflate_assign;
use crate::matrix::rect::rect_i32::RectI32;

#[test]
fn min_bounds() {
    let mut r = RectI32::of(i32::MIN + 7, i32::MIN + 2, i32::MIN + 17, i32::MIN + 13);
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI32::of(i32::MIN + 6, i32::MIN + 1, i32::MIN + 18, i32::MIN + 14));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI32::of(i32::MIN + 5, i32::MIN, i32::MIN + 19, i32::MIN + 15));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI32::of(i32::MIN + 4, i32::MIN, i32::MIN + 20, i32::MIN + 17));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI32::of(i32::MIN + 3, i32::MIN, i32::MIN + 21, i32::MIN + 19));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI32::of(i32::MIN + 2, i32::MIN, i32::MIN + 22, i32::MIN + 21));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI32::of(i32::MIN + 1, i32::MIN, i32::MIN + 23, i32::MIN + 23));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI32::of(i32::MIN, i32::MIN, i32::MIN + 24, i32::MIN + 25));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI32::of(i32::MIN, i32::MIN, i32::MIN + 26, i32::MIN + 27));
}

#[test]
fn max_bounds() {
    let mut r = RectI32::of(i32::MAX - 33, i32::MAX - 17, i32::MAX - 5, i32::MAX - 3);
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI32::of(i32::MAX - 34, i32::MAX - 18, i32::MAX - 4, i32::MAX - 2));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI32::of(i32::MAX - 35, i32::MAX - 19, i32::MAX - 3, i32::MAX - 1));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI32::of(i32::MAX - 36, i32::MAX - 20, i32::MAX - 2, i32::MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI32::of(i32::MAX - 37, i32::MAX - 22, i32::MAX - 1, i32::MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI32::of(i32::MAX - 38, i32::MAX - 24, i32::MAX, i32::MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI32::of(i32::MAX - 40, i32::MAX - 26, i32::MAX, i32::MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI32::of(i32::MAX - 42, i32::MAX - 28, i32::MAX, i32::MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectI32::of(i32::MAX - 44, i32::MAX - 30, i32::MAX, i32::MAX));
}

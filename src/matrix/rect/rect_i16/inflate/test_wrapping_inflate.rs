use super::wrapping_inflate;
use crate::matrix::rect::rect_i16::RectI16;

#[test]
fn min_bounds() {
    assert_eq!(wrapping_inflate(&RectI16::of(i16::MIN + 7, i16::MIN + 3, i16::MIN + 9, i16::MIN + 13)), RectI16::of(i16::MIN + 6, i16::MIN + 2, i16::MIN + 10, i16::MIN + 14));
    assert_eq!(wrapping_inflate(&RectI16::of(i16::MIN + 6, i16::MIN + 2, i16::MIN + 10, i16::MIN + 14)), RectI16::of(i16::MIN + 5, i16::MIN + 1, i16::MIN + 11, i16::MIN + 15));
    assert_eq!(wrapping_inflate(&RectI16::of(i16::MIN + 5, i16::MIN + 1, i16::MIN + 11, i16::MIN + 15)), RectI16::of(i16::MIN + 4, i16::MIN, i16::MIN + 12, i16::MIN + 16));
}

#[test]
fn max_bounds() {
    assert_eq!(wrapping_inflate(&RectI16::of(i16::MAX - 33, i16::MAX - 17, i16::MAX - 5, i16::MAX - 3)), RectI16::of(i16::MAX - 34, i16::MAX - 18, i16::MAX - 4, i16::MAX - 2));
    assert_eq!(wrapping_inflate(&RectI16::of(i16::MAX - 34, i16::MAX - 18, i16::MAX - 4, i16::MAX - 2)), RectI16::of(i16::MAX - 35, i16::MAX - 19, i16::MAX - 3, i16::MAX - 1));
    assert_eq!(wrapping_inflate(&RectI16::of(i16::MAX - 35, i16::MAX - 19, i16::MAX - 3, i16::MAX - 1)), RectI16::of(i16::MAX - 36, i16::MAX - 20, i16::MAX - 2, i16::MAX));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_inflate(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1)), RectI16::largest());
    assert_eq!(wrapping_inflate(&RectI16::of(i16::MIN + 1, i16::MIN + 10, i16::MAX - 10, i16::MAX - 10)), RectI16::of(i16::MIN, i16::MIN + 9, i16::MAX - 9, i16::MAX - 9));
    assert_eq!(wrapping_inflate(&RectI16::of(i16::MIN + 10, i16::MIN + 1, i16::MAX - 10, i16::MAX - 10)), RectI16::of(i16::MIN + 9, i16::MIN, i16::MAX - 9, i16::MAX - 9));
    assert_eq!(wrapping_inflate(&RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 1, i16::MAX - 10)), RectI16::of(i16::MIN + 9, i16::MIN + 9, i16::MAX, i16::MAX - 9));
    assert_eq!(wrapping_inflate(&RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 10, i16::MAX - 1)), RectI16::of(i16::MIN + 9, i16::MIN + 9, i16::MAX - 9, i16::MAX));
}

#[test]
fn out_of_bounds() {
    assert_eq!(wrapping_inflate(&RectI16::largest()), RectI16::of(i16::MAX, i16::MAX, i16::MIN, i16::MIN));
    assert_eq!(wrapping_inflate(&RectI16::of(i16::MIN, i16::MIN + 10, i16::MAX - 10, i16::MAX - 10)), RectI16::of(i16::MAX, i16::MIN + 9, i16::MAX - 9, i16::MAX - 9));
    assert_eq!(wrapping_inflate(&RectI16::of(i16::MIN + 10, i16::MIN, i16::MAX - 10, i16::MAX - 10)), RectI16::of(i16::MIN + 9, i16::MAX, i16::MAX - 9, i16::MAX - 9));
    assert_eq!(wrapping_inflate(&RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX, i16::MAX - 10)), RectI16::of(i16::MIN + 9, i16::MIN + 9, i16::MIN, i16::MAX - 9));
    assert_eq!(wrapping_inflate(&RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 10, i16::MAX)), RectI16::of(i16::MIN + 9, i16::MIN + 9, i16::MAX - 9, i16::MIN));
}

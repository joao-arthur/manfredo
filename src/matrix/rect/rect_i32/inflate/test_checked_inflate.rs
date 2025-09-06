use super::checked_inflate;
use crate::matrix::rect::rect_i32::RectI32;

#[test]
fn min_bounds() {
    assert_eq!(checked_inflate(&RectI32::of(i32::MIN + 7, i32::MIN + 3, i32::MIN + 9, i32::MIN + 13)), RectI32::of(i32::MIN + 6, i32::MIN + 2, i32::MIN + 10, i32::MIN + 14));
    assert_eq!(checked_inflate(&RectI32::of(i32::MIN + 6, i32::MIN + 2, i32::MIN + 10, i32::MIN + 14)), RectI32::of(i32::MIN + 5, i32::MIN + 1, i32::MIN + 11, i32::MIN + 15));
    assert_eq!(checked_inflate(&RectI32::of(i32::MIN + 5, i32::MIN + 1, i32::MIN + 11, i32::MIN + 15)), RectI32::of(i32::MIN + 4, i32::MIN, i32::MIN + 12, i32::MIN + 16));
}

#[test]
fn max_bounds() {
    assert_eq!(checked_inflate(&RectI32::of(i32::MAX - 33, i32::MAX - 17, i32::MAX - 5, i32::MAX - 3)), RectI32::of(i32::MAX - 34, i32::MAX - 18, i32::MAX - 4, i32::MAX - 2));
    assert_eq!(checked_inflate(&RectI32::of(i32::MAX - 34, i32::MAX - 18, i32::MAX - 4, i32::MAX - 2)), RectI32::of(i32::MAX - 35, i32::MAX - 19, i32::MAX - 3, i32::MAX - 1));
    assert_eq!(checked_inflate(&RectI32::of(i32::MAX - 35, i32::MAX - 19, i32::MAX - 3, i32::MAX - 1)), RectI32::of(i32::MAX - 36, i32::MAX - 20, i32::MAX - 2, i32::MAX));
}

use super::saturating_inflate;
use crate::matrix::rect::rect_i32::RectI32;

#[test]
fn min_bounds() {
    assert_eq!(saturating_inflate(&RectI32::of(i32::MIN + 7, i32::MIN + 2, i32::MIN + 17, i32::MIN + 13)), RectI32::of(i32::MIN + 6, i32::MIN + 1, i32::MIN + 18, i32::MIN + 14));
    assert_eq!(saturating_inflate(&RectI32::of(i32::MIN + 6, i32::MIN + 1, i32::MIN + 18, i32::MIN + 14)), RectI32::of(i32::MIN + 5, i32::MIN, i32::MIN + 19, i32::MIN + 15));
    assert_eq!(saturating_inflate(&RectI32::of(i32::MIN + 5, i32::MIN, i32::MIN + 19, i32::MIN + 15)), RectI32::of(i32::MIN + 4, i32::MIN, i32::MIN + 20, i32::MIN + 17));
    assert_eq!(saturating_inflate(&RectI32::of(i32::MIN + 4, i32::MIN, i32::MIN + 20, i32::MIN + 17)), RectI32::of(i32::MIN + 3, i32::MIN, i32::MIN + 21, i32::MIN + 19));
    assert_eq!(saturating_inflate(&RectI32::of(i32::MIN + 3, i32::MIN, i32::MIN + 21, i32::MIN + 19)), RectI32::of(i32::MIN + 2, i32::MIN, i32::MIN + 22, i32::MIN + 21));
    assert_eq!(saturating_inflate(&RectI32::of(i32::MIN + 2, i32::MIN, i32::MIN + 22, i32::MIN + 21)), RectI32::of(i32::MIN + 1, i32::MIN, i32::MIN + 23, i32::MIN + 23));
    assert_eq!(saturating_inflate(&RectI32::of(i32::MIN + 1, i32::MIN, i32::MIN + 23, i32::MIN + 23)), RectI32::of(i32::MIN, i32::MIN, i32::MIN + 24, i32::MIN + 25));
    assert_eq!(saturating_inflate(&RectI32::of(i32::MIN, i32::MIN, i32::MIN + 24, i32::MIN + 25)), RectI32::of(i32::MIN, i32::MIN, i32::MIN + 26, i32::MIN + 27));
}

#[test]
fn max_bounds() {
    assert_eq!(saturating_inflate(&RectI32::of(i32::MAX - 33, i32::MAX - 17, i32::MAX - 5, i32::MAX - 3)), RectI32::of(i32::MAX - 34, i32::MAX - 18, i32::MAX - 4, i32::MAX - 2));
    assert_eq!(saturating_inflate(&RectI32::of(i32::MAX - 34, i32::MAX - 18, i32::MAX - 4, i32::MAX - 2)), RectI32::of(i32::MAX - 35, i32::MAX - 19, i32::MAX - 3, i32::MAX - 1));
    assert_eq!(saturating_inflate(&RectI32::of(i32::MAX - 35, i32::MAX - 19, i32::MAX - 3, i32::MAX - 1)), RectI32::of(i32::MAX - 36, i32::MAX - 20, i32::MAX - 2, i32::MAX));
    assert_eq!(saturating_inflate(&RectI32::of(i32::MAX - 36, i32::MAX - 20, i32::MAX - 2, i32::MAX)), RectI32::of(i32::MAX - 37, i32::MAX - 22, i32::MAX - 1, i32::MAX));
    assert_eq!(saturating_inflate(&RectI32::of(i32::MAX - 37, i32::MAX - 22, i32::MAX - 1, i32::MAX)), RectI32::of(i32::MAX - 38, i32::MAX - 24, i32::MAX, i32::MAX));
    assert_eq!(saturating_inflate(&RectI32::of(i32::MAX - 38, i32::MAX - 24, i32::MAX, i32::MAX)), RectI32::of(i32::MAX - 40, i32::MAX - 26, i32::MAX, i32::MAX));
    assert_eq!(saturating_inflate(&RectI32::of(i32::MAX - 40, i32::MAX - 26, i32::MAX, i32::MAX)), RectI32::of(i32::MAX - 42, i32::MAX - 28, i32::MAX, i32::MAX));
    assert_eq!(saturating_inflate(&RectI32::of(i32::MAX - 42, i32::MAX - 28, i32::MAX, i32::MAX)), RectI32::of(i32::MAX - 44, i32::MAX - 30, i32::MAX, i32::MAX));
}

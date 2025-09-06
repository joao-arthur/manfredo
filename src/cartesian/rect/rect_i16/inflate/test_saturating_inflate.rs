use super::saturating_inflate;
use crate::cartesian::rect::rect_i16::RectI16;

#[test]
fn min_bounds() {
    assert_eq!(saturating_inflate(&RectI16::of(i16::MIN + 7, i16::MIN + 2, i16::MIN + 17, i16::MIN + 13)), RectI16::of(i16::MIN + 6, i16::MIN + 1, i16::MIN + 18, i16::MIN + 14));
    assert_eq!(saturating_inflate(&RectI16::of(i16::MIN + 6, i16::MIN + 1, i16::MIN + 18, i16::MIN + 14)), RectI16::of(i16::MIN + 5, i16::MIN, i16::MIN + 19, i16::MIN + 15));
    assert_eq!(saturating_inflate(&RectI16::of(i16::MIN + 5, i16::MIN, i16::MIN + 19, i16::MIN + 15)), RectI16::of(i16::MIN + 4, i16::MIN, i16::MIN + 20, i16::MIN + 17));
    assert_eq!(saturating_inflate(&RectI16::of(i16::MIN + 4, i16::MIN, i16::MIN + 20, i16::MIN + 17)), RectI16::of(i16::MIN + 3, i16::MIN, i16::MIN + 21, i16::MIN + 19));
    assert_eq!(saturating_inflate(&RectI16::of(i16::MIN + 3, i16::MIN, i16::MIN + 21, i16::MIN + 19)), RectI16::of(i16::MIN + 2, i16::MIN, i16::MIN + 22, i16::MIN + 21));
    assert_eq!(saturating_inflate(&RectI16::of(i16::MIN + 2, i16::MIN, i16::MIN + 22, i16::MIN + 21)), RectI16::of(i16::MIN + 1, i16::MIN, i16::MIN + 23, i16::MIN + 23));
    assert_eq!(saturating_inflate(&RectI16::of(i16::MIN + 1, i16::MIN, i16::MIN + 23, i16::MIN + 23)), RectI16::of(i16::MIN, i16::MIN, i16::MIN + 24, i16::MIN + 25));
    assert_eq!(saturating_inflate(&RectI16::of(i16::MIN, i16::MIN, i16::MIN + 24, i16::MIN + 25)), RectI16::of(i16::MIN, i16::MIN, i16::MIN + 26, i16::MIN + 27));
}

#[test]
fn max_bounds() {
    assert_eq!(saturating_inflate(&RectI16::of(i16::MAX - 33, i16::MAX - 17, i16::MAX - 5, i16::MAX - 3)), RectI16::of(i16::MAX - 34, i16::MAX - 18, i16::MAX - 4, i16::MAX - 2));
    assert_eq!(saturating_inflate(&RectI16::of(i16::MAX - 34, i16::MAX - 18, i16::MAX - 4, i16::MAX - 2)), RectI16::of(i16::MAX - 35, i16::MAX - 19, i16::MAX - 3, i16::MAX - 1));
    assert_eq!(saturating_inflate(&RectI16::of(i16::MAX - 35, i16::MAX - 19, i16::MAX - 3, i16::MAX - 1)), RectI16::of(i16::MAX - 36, i16::MAX - 20, i16::MAX - 2, i16::MAX));
    assert_eq!(saturating_inflate(&RectI16::of(i16::MAX - 36, i16::MAX - 20, i16::MAX - 2, i16::MAX)), RectI16::of(i16::MAX - 37, i16::MAX - 22, i16::MAX - 1, i16::MAX));
    assert_eq!(saturating_inflate(&RectI16::of(i16::MAX - 37, i16::MAX - 22, i16::MAX - 1, i16::MAX)), RectI16::of(i16::MAX - 38, i16::MAX - 24, i16::MAX, i16::MAX));
    assert_eq!(saturating_inflate(&RectI16::of(i16::MAX - 38, i16::MAX - 24, i16::MAX, i16::MAX)), RectI16::of(i16::MAX - 40, i16::MAX - 26, i16::MAX, i16::MAX));
    assert_eq!(saturating_inflate(&RectI16::of(i16::MAX - 40, i16::MAX - 26, i16::MAX, i16::MAX)), RectI16::of(i16::MAX - 42, i16::MAX - 28, i16::MAX, i16::MAX));
    assert_eq!(saturating_inflate(&RectI16::of(i16::MAX - 42, i16::MAX - 28, i16::MAX, i16::MAX)), RectI16::of(i16::MAX - 44, i16::MAX - 30, i16::MAX, i16::MAX));
}

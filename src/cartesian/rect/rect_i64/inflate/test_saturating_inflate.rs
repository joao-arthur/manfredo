use super::saturating_inflate;
use crate::cartesian::rect::rect_i64::RectI64;

const MIN: i64 = i64::MIN;
const MAX: i64 = i64::MAX;

#[test]
fn min_bounds() {
    assert_eq!(saturating_inflate(&RectI64::of(MIN + 7, MIN + 2, MIN + 17, MIN + 13)), RectI64::of(MIN + 6, MIN + 1, MIN + 18, MIN + 14));
    assert_eq!(saturating_inflate(&RectI64::of(MIN + 6, MIN + 1, MIN + 18, MIN + 14)), RectI64::of(MIN + 5, MIN, MIN + 19, MIN + 15));
    assert_eq!(saturating_inflate(&RectI64::of(MIN + 5, MIN, MIN + 19, MIN + 15)), RectI64::of(MIN + 4, MIN, MIN + 20, MIN + 17));
    assert_eq!(saturating_inflate(&RectI64::of(MIN + 4, MIN, MIN + 20, MIN + 17)), RectI64::of(MIN + 3, MIN, MIN + 21, MIN + 19));
    assert_eq!(saturating_inflate(&RectI64::of(MIN + 3, MIN, MIN + 21, MIN + 19)), RectI64::of(MIN + 2, MIN, MIN + 22, MIN + 21));
    assert_eq!(saturating_inflate(&RectI64::of(MIN + 2, MIN, MIN + 22, MIN + 21)), RectI64::of(MIN + 1, MIN, MIN + 23, MIN + 23));
    assert_eq!(saturating_inflate(&RectI64::of(MIN + 1, MIN, MIN + 23, MIN + 23)), RectI64::of(MIN, MIN, MIN + 24, MIN + 25));
    assert_eq!(saturating_inflate(&RectI64::of(MIN, MIN, MIN + 24, MIN + 25)), RectI64::of(MIN, MIN, MIN + 26, MIN + 27));
}

#[test]
fn max_bounds() {
    assert_eq!(saturating_inflate(&RectI64::of(MAX - 33, MAX - 17, MAX - 5, MAX - 3)), RectI64::of(MAX - 34, MAX - 18, MAX - 4, MAX - 2));
    assert_eq!(saturating_inflate(&RectI64::of(MAX - 34, MAX - 18, MAX - 4, MAX - 2)), RectI64::of(MAX - 35, MAX - 19, MAX - 3, MAX - 1));
    assert_eq!(saturating_inflate(&RectI64::of(MAX - 35, MAX - 19, MAX - 3, MAX - 1)), RectI64::of(MAX - 36, MAX - 20, MAX - 2, MAX));
    assert_eq!(saturating_inflate(&RectI64::of(MAX - 36, MAX - 20, MAX - 2, MAX)), RectI64::of(MAX - 37, MAX - 22, MAX - 1, MAX));
    assert_eq!(saturating_inflate(&RectI64::of(MAX - 37, MAX - 22, MAX - 1, MAX)), RectI64::of(MAX - 38, MAX - 24, MAX, MAX));
    assert_eq!(saturating_inflate(&RectI64::of(MAX - 38, MAX - 24, MAX, MAX)), RectI64::of(MAX - 40, MAX - 26, MAX, MAX));
    assert_eq!(saturating_inflate(&RectI64::of(MAX - 40, MAX - 26, MAX, MAX)), RectI64::of(MAX - 42, MAX - 28, MAX, MAX));
    assert_eq!(saturating_inflate(&RectI64::of(MAX - 42, MAX - 28, MAX, MAX)), RectI64::of(MAX - 44, MAX - 30, MAX, MAX));
}

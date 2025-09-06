use super::saturating_inflate;
use crate::cartesian::rect::rect_i32::RectI32;

const MIN: i32 = i32::MIN;
const MAX: i32 = i32::MAX;

#[test]
fn min_bounds() {
    assert_eq!(saturating_inflate(&RectI32::of(MIN + 7, MIN + 2, MIN + 17, MIN + 13)), RectI32::of(MIN + 6, MIN + 1, MIN + 18, MIN + 14));
    assert_eq!(saturating_inflate(&RectI32::of(MIN + 6, MIN + 1, MIN + 18, MIN + 14)), RectI32::of(MIN + 5, MIN, MIN + 19, MIN + 15));
    assert_eq!(saturating_inflate(&RectI32::of(MIN + 5, MIN, MIN + 19, MIN + 15)), RectI32::of(MIN + 4, MIN, MIN + 20, MIN + 17));
    assert_eq!(saturating_inflate(&RectI32::of(MIN + 4, MIN, MIN + 20, MIN + 17)), RectI32::of(MIN + 3, MIN, MIN + 21, MIN + 19));
    assert_eq!(saturating_inflate(&RectI32::of(MIN + 3, MIN, MIN + 21, MIN + 19)), RectI32::of(MIN + 2, MIN, MIN + 22, MIN + 21));
    assert_eq!(saturating_inflate(&RectI32::of(MIN + 2, MIN, MIN + 22, MIN + 21)), RectI32::of(MIN + 1, MIN, MIN + 23, MIN + 23));
    assert_eq!(saturating_inflate(&RectI32::of(MIN + 1, MIN, MIN + 23, MIN + 23)), RectI32::of(MIN, MIN, MIN + 24, MIN + 25));
    assert_eq!(saturating_inflate(&RectI32::of(MIN, MIN, MIN + 24, MIN + 25)), RectI32::of(MIN, MIN, MIN + 26, MIN + 27));
}

#[test]
fn max_bounds() {
    assert_eq!(saturating_inflate(&RectI32::of(MAX - 33, MAX - 17, MAX - 5, MAX - 3)), RectI32::of(MAX - 34, MAX - 18, MAX - 4, MAX - 2));
    assert_eq!(saturating_inflate(&RectI32::of(MAX - 34, MAX - 18, MAX - 4, MAX - 2)), RectI32::of(MAX - 35, MAX - 19, MAX - 3, MAX - 1));
    assert_eq!(saturating_inflate(&RectI32::of(MAX - 35, MAX - 19, MAX - 3, MAX - 1)), RectI32::of(MAX - 36, MAX - 20, MAX - 2, MAX));
    assert_eq!(saturating_inflate(&RectI32::of(MAX - 36, MAX - 20, MAX - 2, MAX)), RectI32::of(MAX - 37, MAX - 22, MAX - 1, MAX));
    assert_eq!(saturating_inflate(&RectI32::of(MAX - 37, MAX - 22, MAX - 1, MAX)), RectI32::of(MAX - 38, MAX - 24, MAX, MAX));
    assert_eq!(saturating_inflate(&RectI32::of(MAX - 38, MAX - 24, MAX, MAX)), RectI32::of(MAX - 40, MAX - 26, MAX, MAX));
    assert_eq!(saturating_inflate(&RectI32::of(MAX - 40, MAX - 26, MAX, MAX)), RectI32::of(MAX - 42, MAX - 28, MAX, MAX));
    assert_eq!(saturating_inflate(&RectI32::of(MAX - 42, MAX - 28, MAX, MAX)), RectI32::of(MAX - 44, MAX - 30, MAX, MAX));
}

use super::saturating_inflate;
use crate::matrix::rect::rect_u64::RectU64;

#[test]
fn min_bounds() {
    assert_eq!(saturating_inflate(&RectU64::of(7, 2, 17, 13)), RectU64::of(6, 1, 18, 14));
    assert_eq!(saturating_inflate(&RectU64::of(6, 1, 18, 14)), RectU64::of(5, 0, 19, 15));
    assert_eq!(saturating_inflate(&RectU64::of(5, 0, 19, 15)), RectU64::of(4, 0, 20, 17));
    assert_eq!(saturating_inflate(&RectU64::of(4, 0, 20, 17)), RectU64::of(3, 0, 21, 19));
    assert_eq!(saturating_inflate(&RectU64::of(3, 0, 21, 19)), RectU64::of(2, 0, 22, 21));
    assert_eq!(saturating_inflate(&RectU64::of(2, 0, 22, 21)), RectU64::of(1, 0, 23, 23));
    assert_eq!(saturating_inflate(&RectU64::of(1, 0, 23, 23)), RectU64::of(0, 0, 24, 25));
    assert_eq!(saturating_inflate(&RectU64::of(0, 0, 24, 25)), RectU64::of(0, 0, 26, 27));
}

#[test]
fn max_bounds() {
    assert_eq!(saturating_inflate(&RectU64::of(u64::MAX - 33, u64::MAX - 17, u64::MAX - 5, u64::MAX - 3)), RectU64::of(u64::MAX - 34, u64::MAX - 18, u64::MAX - 4, u64::MAX - 2));
    assert_eq!(saturating_inflate(&RectU64::of(u64::MAX - 34, u64::MAX - 18, u64::MAX - 4, u64::MAX - 2)), RectU64::of(u64::MAX - 35, u64::MAX - 19, u64::MAX - 3, u64::MAX - 1));
    assert_eq!(saturating_inflate(&RectU64::of(u64::MAX - 35, u64::MAX - 19, u64::MAX - 3, u64::MAX - 1)), RectU64::of(u64::MAX - 36, u64::MAX - 20, u64::MAX - 2, u64::MAX));
    assert_eq!(saturating_inflate(&RectU64::of(u64::MAX - 36, u64::MAX - 20, u64::MAX - 2, u64::MAX)), RectU64::of(u64::MAX - 37, u64::MAX - 22, u64::MAX - 1, u64::MAX));
    assert_eq!(saturating_inflate(&RectU64::of(u64::MAX - 37, u64::MAX - 22, u64::MAX - 1, u64::MAX)), RectU64::of(u64::MAX - 38, u64::MAX - 24, u64::MAX, u64::MAX));
    assert_eq!(saturating_inflate(&RectU64::of(u64::MAX - 38, u64::MAX - 24, u64::MAX, u64::MAX)), RectU64::of(u64::MAX - 40, u64::MAX - 26, u64::MAX, u64::MAX));
    assert_eq!(saturating_inflate(&RectU64::of(u64::MAX - 40, u64::MAX - 26, u64::MAX, u64::MAX)), RectU64::of(u64::MAX - 42, u64::MAX - 28, u64::MAX, u64::MAX));
    assert_eq!(saturating_inflate(&RectU64::of(u64::MAX - 42, u64::MAX - 28, u64::MAX, u64::MAX)), RectU64::of(u64::MAX - 44, u64::MAX - 30, u64::MAX, u64::MAX));
}

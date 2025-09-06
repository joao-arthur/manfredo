use super::checked_inflate;
use crate::matrix::rect::rect_u64::RectU64;

#[test]
fn checked_inflate_min_bounds() {
    assert_eq!(checked_inflate(&RectU64::of(7, 3, 9, 13)), RectU64::of(6, 2, 10, 14));
    assert_eq!(checked_inflate(&RectU64::of(6, 2, 10, 14)), RectU64::of(5, 1, 11, 15));
    assert_eq!(checked_inflate(&RectU64::of(5, 1, 11, 15)), RectU64::of(4, 0, 12, 16));
}

#[test]
fn checked_inflate_max_bounds() {
    assert_eq!(checked_inflate(&RectU64::of(u64::MAX - 33, u64::MAX - 17, u64::MAX - 5, u64::MAX - 3)), RectU64::of(u64::MAX - 34, u64::MAX - 18, u64::MAX - 4, u64::MAX - 2));
    assert_eq!(checked_inflate(&RectU64::of(u64::MAX - 34, u64::MAX - 18, u64::MAX - 4, u64::MAX - 2)), RectU64::of(u64::MAX - 35, u64::MAX - 19, u64::MAX - 3, u64::MAX - 1));
    assert_eq!(checked_inflate(&RectU64::of(u64::MAX - 35, u64::MAX - 19, u64::MAX - 3, u64::MAX - 1)), RectU64::of(u64::MAX - 36, u64::MAX - 20, u64::MAX - 2, u64::MAX));
}

use super::checked_resize;
use crate::cartesian::rect::rect_u64::RectU64;

#[test]
fn checked_resize_odd() {
    assert_eq!(checked_resize(&RectU64::of(5, 5, 15, 15), 9), RectU64::of(6, 6, 14, 14));
    assert_eq!(checked_resize(&RectU64::of(6, 6, 14, 14), 7), RectU64::of(7, 7, 13, 13));
    assert_eq!(checked_resize(&RectU64::of(7, 7, 13, 13), 5), RectU64::of(8, 8, 12, 12));
    assert_eq!(checked_resize(&RectU64::of(8, 8, 12, 12), 3), RectU64::of(9, 9, 11, 11));
    assert_eq!(checked_resize(&RectU64::of(9, 9, 11, 11), 9), RectU64::of(6, 6, 14, 14));
}

#[test]
fn checked_resize_even() {
    assert_eq!(checked_resize(&RectU64::of(5, 5, 14, 14), 10), RectU64::of(5, 5, 14, 14));
    assert_eq!(checked_resize(&RectU64::of(5, 5, 14, 14), 8), RectU64::of(6, 6, 13, 13));
    assert_eq!(checked_resize(&RectU64::of(6, 6, 13, 13), 6), RectU64::of(7, 7, 12, 12));
    assert_eq!(checked_resize(&RectU64::of(7, 7, 12, 12), 4), RectU64::of(8, 8, 11, 11));
    assert_eq!(checked_resize(&RectU64::of(8, 8, 11, 11), 8), RectU64::of(6, 6, 13, 13));
}

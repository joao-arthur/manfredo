use super::wrapping_resize;
use crate::cartesian::rect::rect_i64::RectI64;

#[test]
fn wrapping_resize_odd() {
    assert_eq!(wrapping_resize(&RectI64::of(5, 5, 15, 15), 9), RectI64::of(6, 6, 14, 14));
    assert_eq!(wrapping_resize(&RectI64::of(6, 6, 14, 14), 7), RectI64::of(7, 7, 13, 13));
    assert_eq!(wrapping_resize(&RectI64::of(7, 7, 13, 13), 5), RectI64::of(8, 8, 12, 12));
    assert_eq!(wrapping_resize(&RectI64::of(8, 8, 12, 12), 3), RectI64::of(9, 9, 11, 11));
    assert_eq!(wrapping_resize(&RectI64::of(9, 9, 11, 11), 9), RectI64::of(6, 6, 14, 14));
}

#[test]
fn wrapping_resize_even() {
    assert_eq!(wrapping_resize(&RectI64::of(5, 5, 14, 14), 10), RectI64::of(5, 5, 14, 14));
    assert_eq!(wrapping_resize(&RectI64::of(5, 5, 14, 14), 8), RectI64::of(6, 6, 13, 13));
    assert_eq!(wrapping_resize(&RectI64::of(6, 6, 13, 13), 6), RectI64::of(7, 7, 12, 12));
    assert_eq!(wrapping_resize(&RectI64::of(7, 7, 12, 12), 4), RectI64::of(8, 8, 11, 11));
    assert_eq!(wrapping_resize(&RectI64::of(8, 8, 11, 11), 8), RectI64::of(6, 6, 13, 13));
}

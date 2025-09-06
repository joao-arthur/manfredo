use super::checked_inflate;
use crate::matrix::rect::rect_u32::RectU32;

#[test]
fn checked_inflate_min_bounds() {
    assert_eq!(checked_inflate(&RectU32::of(7, 3, 9, 13)), RectU32::of(6, 2, 10, 14));
    assert_eq!(checked_inflate(&RectU32::of(6, 2, 10, 14)), RectU32::of(5, 1, 11, 15));
    assert_eq!(checked_inflate(&RectU32::of(5, 1, 11, 15)), RectU32::of(4, 0, 12, 16));
}

#[test]
fn checked_inflate_max_bounds() {
    assert_eq!(checked_inflate(&RectU32::of(u32::MAX - 33, u32::MAX - 17, u32::MAX - 5, u32::MAX - 3)), RectU32::of(u32::MAX - 34, u32::MAX - 18, u32::MAX - 4, u32::MAX - 2));
    assert_eq!(checked_inflate(&RectU32::of(u32::MAX - 34, u32::MAX - 18, u32::MAX - 4, u32::MAX - 2)), RectU32::of(u32::MAX - 35, u32::MAX - 19, u32::MAX - 3, u32::MAX - 1));
    assert_eq!(checked_inflate(&RectU32::of(u32::MAX - 35, u32::MAX - 19, u32::MAX - 3, u32::MAX - 1)), RectU32::of(u32::MAX - 36, u32::MAX - 20, u32::MAX - 2, u32::MAX));
}

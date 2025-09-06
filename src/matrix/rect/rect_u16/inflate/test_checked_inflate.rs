use super::checked_inflate;
use crate::matrix::rect::rect_u16::RectU16;

#[test]
fn checked_inflate_min_bounds() {
    assert_eq!(checked_inflate(&RectU16::of(7, 3, 9, 13)), RectU16::of(6, 2, 10, 14));
    assert_eq!(checked_inflate(&RectU16::of(6, 2, 10, 14)), RectU16::of(5, 1, 11, 15));
    assert_eq!(checked_inflate(&RectU16::of(5, 1, 11, 15)), RectU16::of(4, 0, 12, 16));
}

#[test]
fn checked_inflate_max_bounds() {
    assert_eq!(checked_inflate(&RectU16::of(u16::MAX - 33, u16::MAX - 17, u16::MAX - 5, u16::MAX - 3)), RectU16::of(u16::MAX - 34, u16::MAX - 18, u16::MAX - 4, u16::MAX - 2));
    assert_eq!(checked_inflate(&RectU16::of(u16::MAX - 34, u16::MAX - 18, u16::MAX - 4, u16::MAX - 2)), RectU16::of(u16::MAX - 35, u16::MAX - 19, u16::MAX - 3, u16::MAX - 1));
    assert_eq!(checked_inflate(&RectU16::of(u16::MAX - 35, u16::MAX - 19, u16::MAX - 3, u16::MAX - 1)), RectU16::of(u16::MAX - 36, u16::MAX - 20, u16::MAX - 2, u16::MAX));
}

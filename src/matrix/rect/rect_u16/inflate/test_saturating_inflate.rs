use super::saturating_inflate;
use crate::matrix::rect::rect_u16::RectU16;

#[test]
fn min_bounds() {
    assert_eq!(saturating_inflate(&RectU16::of(7, 2, 17, 13)), RectU16::of(6, 1, 18, 14));
    assert_eq!(saturating_inflate(&RectU16::of(6, 1, 18, 14)), RectU16::of(5, 0, 19, 15));
    assert_eq!(saturating_inflate(&RectU16::of(5, 0, 19, 15)), RectU16::of(4, 0, 20, 17));
    assert_eq!(saturating_inflate(&RectU16::of(4, 0, 20, 17)), RectU16::of(3, 0, 21, 19));
    assert_eq!(saturating_inflate(&RectU16::of(3, 0, 21, 19)), RectU16::of(2, 0, 22, 21));
    assert_eq!(saturating_inflate(&RectU16::of(2, 0, 22, 21)), RectU16::of(1, 0, 23, 23));
    assert_eq!(saturating_inflate(&RectU16::of(1, 0, 23, 23)), RectU16::of(0, 0, 24, 25));
    assert_eq!(saturating_inflate(&RectU16::of(0, 0, 24, 25)), RectU16::of(0, 0, 26, 27));
}

#[test]
fn max_bounds() {
    assert_eq!(saturating_inflate(&RectU16::of(u16::MAX - 33, u16::MAX - 17, u16::MAX - 5, u16::MAX - 3)), RectU16::of(u16::MAX - 34, u16::MAX - 18, u16::MAX - 4, u16::MAX - 2));
    assert_eq!(saturating_inflate(&RectU16::of(u16::MAX - 34, u16::MAX - 18, u16::MAX - 4, u16::MAX - 2)), RectU16::of(u16::MAX - 35, u16::MAX - 19, u16::MAX - 3, u16::MAX - 1));
    assert_eq!(saturating_inflate(&RectU16::of(u16::MAX - 35, u16::MAX - 19, u16::MAX - 3, u16::MAX - 1)), RectU16::of(u16::MAX - 36, u16::MAX - 20, u16::MAX - 2, u16::MAX));
    assert_eq!(saturating_inflate(&RectU16::of(u16::MAX - 36, u16::MAX - 20, u16::MAX - 2, u16::MAX)), RectU16::of(u16::MAX - 37, u16::MAX - 22, u16::MAX - 1, u16::MAX));
    assert_eq!(saturating_inflate(&RectU16::of(u16::MAX - 37, u16::MAX - 22, u16::MAX - 1, u16::MAX)), RectU16::of(u16::MAX - 38, u16::MAX - 24, u16::MAX, u16::MAX));
    assert_eq!(saturating_inflate(&RectU16::of(u16::MAX - 38, u16::MAX - 24, u16::MAX, u16::MAX)), RectU16::of(u16::MAX - 40, u16::MAX - 26, u16::MAX, u16::MAX));
    assert_eq!(saturating_inflate(&RectU16::of(u16::MAX - 40, u16::MAX - 26, u16::MAX, u16::MAX)), RectU16::of(u16::MAX - 42, u16::MAX - 28, u16::MAX, u16::MAX));
    assert_eq!(saturating_inflate(&RectU16::of(u16::MAX - 42, u16::MAX - 28, u16::MAX, u16::MAX)), RectU16::of(u16::MAX - 44, u16::MAX - 30, u16::MAX, u16::MAX));
}

use super::saturating_inflate;
use crate::matrix::rect::rect_u16::RectU16;

const MAX: u16 = u16::MAX;

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
    assert_eq!(saturating_inflate(&RectU16::of(MAX - 33, MAX - 17, MAX - 5, MAX - 3)), RectU16::of(MAX - 34, MAX - 18, MAX - 4, MAX - 2));
    assert_eq!(saturating_inflate(&RectU16::of(MAX - 34, MAX - 18, MAX - 4, MAX - 2)), RectU16::of(MAX - 35, MAX - 19, MAX - 3, MAX - 1));
    assert_eq!(saturating_inflate(&RectU16::of(MAX - 35, MAX - 19, MAX - 3, MAX - 1)), RectU16::of(MAX - 36, MAX - 20, MAX - 2, MAX));
    assert_eq!(saturating_inflate(&RectU16::of(MAX - 36, MAX - 20, MAX - 2, MAX)), RectU16::of(MAX - 37, MAX - 22, MAX - 1, MAX));
    assert_eq!(saturating_inflate(&RectU16::of(MAX - 37, MAX - 22, MAX - 1, MAX)), RectU16::of(MAX - 38, MAX - 24, MAX, MAX));
    assert_eq!(saturating_inflate(&RectU16::of(MAX - 38, MAX - 24, MAX, MAX)), RectU16::of(MAX - 40, MAX - 26, MAX, MAX));
    assert_eq!(saturating_inflate(&RectU16::of(MAX - 40, MAX - 26, MAX, MAX)), RectU16::of(MAX - 42, MAX - 28, MAX, MAX));
    assert_eq!(saturating_inflate(&RectU16::of(MAX - 42, MAX - 28, MAX, MAX)), RectU16::of(MAX - 44, MAX - 30, MAX, MAX));
}

use super::saturating_inflate;
use crate::matrix::rect::rect_u8::RectU8;

#[test]
fn saturating_inflate_min_bounds() {
    assert_eq!(saturating_inflate(&RectU8::of(7, 2, 17, 13)), RectU8::of(6, 1, 18, 14));
    assert_eq!(saturating_inflate(&RectU8::of(6, 1, 18, 14)), RectU8::of(5, 0, 19, 15));
    assert_eq!(saturating_inflate(&RectU8::of(5, 0, 19, 15)), RectU8::of(4, 0, 20, 17));
    assert_eq!(saturating_inflate(&RectU8::of(4, 0, 20, 17)), RectU8::of(3, 0, 21, 19));
    assert_eq!(saturating_inflate(&RectU8::of(3, 0, 21, 19)), RectU8::of(2, 0, 22, 21));
    assert_eq!(saturating_inflate(&RectU8::of(2, 0, 22, 21)), RectU8::of(1, 0, 23, 23));
    assert_eq!(saturating_inflate(&RectU8::of(1, 0, 23, 23)), RectU8::of(0, 0, 24, 25));
    assert_eq!(saturating_inflate(&RectU8::of(0, 0, 24, 25)), RectU8::of(0, 0, 26, 27));
}

#[test]
fn saturating_inflate_max_bounds() {
    assert_eq!(saturating_inflate(&RectU8::of(u8::MAX - 33, u8::MAX - 17, u8::MAX - 5, u8::MAX - 3)), RectU8::of(u8::MAX - 34, u8::MAX - 18, u8::MAX - 4, u8::MAX - 2));
    assert_eq!(saturating_inflate(&RectU8::of(u8::MAX - 34, u8::MAX - 18, u8::MAX - 4, u8::MAX - 2)), RectU8::of(u8::MAX - 35, u8::MAX - 19, u8::MAX - 3, u8::MAX - 1));
    assert_eq!(saturating_inflate(&RectU8::of(u8::MAX - 35, u8::MAX - 19, u8::MAX - 3, u8::MAX - 1)), RectU8::of(u8::MAX - 36, u8::MAX - 20, u8::MAX - 2, u8::MAX));
    assert_eq!(saturating_inflate(&RectU8::of(u8::MAX - 36, u8::MAX - 20, u8::MAX - 2, u8::MAX)), RectU8::of(u8::MAX - 37, u8::MAX - 22, u8::MAX - 1, u8::MAX));
    assert_eq!(saturating_inflate(&RectU8::of(u8::MAX - 37, u8::MAX - 22, u8::MAX - 1, u8::MAX)), RectU8::of(u8::MAX - 38, u8::MAX - 24, u8::MAX, u8::MAX));
    assert_eq!(saturating_inflate(&RectU8::of(u8::MAX - 38, u8::MAX - 24, u8::MAX, u8::MAX)), RectU8::of(u8::MAX - 40, u8::MAX - 26, u8::MAX, u8::MAX));
    assert_eq!(saturating_inflate(&RectU8::of(u8::MAX - 40, u8::MAX - 26, u8::MAX, u8::MAX)), RectU8::of(u8::MAX - 42, u8::MAX - 28, u8::MAX, u8::MAX));
    assert_eq!(saturating_inflate(&RectU8::of(u8::MAX - 42, u8::MAX - 28, u8::MAX, u8::MAX)), RectU8::of(u8::MAX - 44, u8::MAX - 30, u8::MAX, u8::MAX));
}

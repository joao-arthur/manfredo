use super::checked_inflate;
use crate::cartesian::rect::rect_u8::RectU8;

const MAX: u8 = u8::MAX;

#[test]
fn min_bounds() {
    assert_eq!(checked_inflate(&RectU8::of(7, 3, 9, 13)), RectU8::of(6, 2, 10, 14));
    assert_eq!(checked_inflate(&RectU8::of(6, 2, 10, 14)), RectU8::of(5, 1, 11, 15));
    assert_eq!(checked_inflate(&RectU8::of(5, 1, 11, 15)), RectU8::of(4, 0, 12, 16));
}

#[test]
fn max_bounds() {
    assert_eq!(checked_inflate(&RectU8::of(MAX - 33, MAX - 17, MAX - 5, MAX - 3)), RectU8::of(MAX - 34, MAX - 18, MAX - 4, MAX - 2));
    assert_eq!(checked_inflate(&RectU8::of(MAX - 34, MAX - 18, MAX - 4, MAX - 2)), RectU8::of(MAX - 35, MAX - 19, MAX - 3, MAX - 1));
    assert_eq!(checked_inflate(&RectU8::of(MAX - 35, MAX - 19, MAX - 3, MAX - 1)), RectU8::of(MAX - 36, MAX - 20, MAX - 2, MAX));
}

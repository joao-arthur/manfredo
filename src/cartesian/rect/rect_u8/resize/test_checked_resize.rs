use super::checked_resize;
use crate::cartesian::rect::rect_u8::RectU8;

#[test]
fn odd() {
    assert_eq!(checked_resize(&RectU8::of(5, 5, 15, 15), 9), RectU8::of(6, 6, 14, 14));
    assert_eq!(checked_resize(&RectU8::of(6, 6, 14, 14), 7), RectU8::of(7, 7, 13, 13));
    assert_eq!(checked_resize(&RectU8::of(7, 7, 13, 13), 5), RectU8::of(8, 8, 12, 12));
    assert_eq!(checked_resize(&RectU8::of(8, 8, 12, 12), 3), RectU8::of(9, 9, 11, 11));
    assert_eq!(checked_resize(&RectU8::of(9, 9, 11, 11), 9), RectU8::of(6, 6, 14, 14));
}

#[test]
fn even() {
    assert_eq!(checked_resize(&RectU8::of(5, 5, 14, 14), 10), RectU8::of(5, 5, 14, 14));
    assert_eq!(checked_resize(&RectU8::of(5, 5, 14, 14), 8), RectU8::of(6, 6, 13, 13));
    assert_eq!(checked_resize(&RectU8::of(6, 6, 13, 13), 6), RectU8::of(7, 7, 12, 12));
    assert_eq!(checked_resize(&RectU8::of(7, 7, 12, 12), 4), RectU8::of(8, 8, 11, 11));
    assert_eq!(checked_resize(&RectU8::of(8, 8, 11, 11), 8), RectU8::of(6, 6, 13, 13));
}

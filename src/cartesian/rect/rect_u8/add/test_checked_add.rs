use super::checked_add;
use crate::cartesian::rect::{rect_i8::Rect as RectI, rect_u8::RectU8};

#[test]
fn test() {
    assert_eq!(checked_add(&RectU8::of(0, 0, 12, 15), &RectI::of(5, 4, 3, 2)), RectU8::of(5, 4, 15, 17));
    assert_eq!(checked_add(&RectU8::of(5, 4, 15, 20), &RectI::of(-4, -3, -2, -1)), RectU8::of(1, 1, 13, 19));
}

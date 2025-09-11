use super::checked_add;
use crate::matrix::rect::{rect_i16::Rect, rect_u16::RectU16};

#[test]
fn test() {
    assert_eq!(checked_add(&RectU16::of(0, 0, 12, 15), &Rect::of(5, 4, 3, 2)), RectU16::of(5, 4, 15, 17));
    assert_eq!(checked_add(&RectU16::of(5, 4, 15, 20), &Rect::of(-4, -3, -2, -1)), RectU16::of(1, 1, 13, 19));
}

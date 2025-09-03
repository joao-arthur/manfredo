use super::checked_translate;
use crate::matrix::{point::point_i16::PointI16, rect::rect_u16::RectU16};

#[test]
fn test_checked_translate() {
    assert_eq!(checked_translate(&RectU16::of(0, 0, 12, 15), &PointI16::of(5, 4)), RectU16::of(5, 4, 17, 19));
    assert_eq!(checked_translate(&RectU16::of(5, 4, 17, 19), &PointI16::of(-4, -2)), RectU16::of(1, 2, 13, 17));
}

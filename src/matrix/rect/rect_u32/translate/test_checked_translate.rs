use super::checked_translate;
use crate::matrix::{point::point_i32::PointI32, rect::rect_u32::RectU32};

#[test]
fn test() {
    assert_eq!(checked_translate(&RectU32::of(0, 0, 12, 15), &PointI32::of(5, 4)), RectU32::of(5, 4, 17, 19));
    assert_eq!(checked_translate(&RectU32::of(5, 4, 17, 19), &PointI32::of(-4, -2)), RectU32::of(1, 2, 13, 17));
}

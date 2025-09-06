use super::checked_translate;
use crate::cartesian::{point::point_i32::PointI32, rect::rect_i32::RectI32};

#[test]
fn test() {
    assert_eq!(checked_translate(&RectI32::of(5, 9, 13, 37), &PointI32::of(-10, -20)), RectI32::of(-5, -11, 3, 17));
    assert_eq!(checked_translate(&RectI32::of(-5, -11, 3, 17), &PointI32::of(6, -19)), RectI32::of(1, -30, 9, -2));
}

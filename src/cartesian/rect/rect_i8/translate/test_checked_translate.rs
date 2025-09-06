use super::checked_translate;
use crate::cartesian::{point::point_i8::PointI8, rect::rect_i8::RectI8};

#[test]
fn test() {
    assert_eq!(checked_translate(&RectI8::of(5, 9, 13, 37), &PointI8::of(-10, -20)), RectI8::of(-5, -11, 3, 17));
    assert_eq!(checked_translate(&RectI8::of(-5, -11, 3, 17), &PointI8::of(6, -19)), RectI8::of(1, -30, 9, -2));
}

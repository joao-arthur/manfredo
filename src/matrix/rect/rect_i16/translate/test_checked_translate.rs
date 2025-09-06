use super::checked_translate;
use crate::matrix::{point::point_i16::PointI16, rect::rect_i16::RectI16};

#[test]
fn test() {
    assert_eq!(checked_translate(&RectI16::of(5, 9, 13, 37), &PointI16::of(-10, -20)), RectI16::of(-5, -11, 3, 17));
    assert_eq!(checked_translate(&RectI16::of(-5, -11, 3, 17), &PointI16::of(6, -19)), RectI16::of(1, -30, 9, -2));
}

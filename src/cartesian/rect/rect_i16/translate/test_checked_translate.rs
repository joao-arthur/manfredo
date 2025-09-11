use super::checked_translate;
use crate::cartesian::{point::point_i16::PointI16, rect::rect_i16::Rect};

#[test]
fn test() {
    assert_eq!(checked_translate(&Rect::of(5, 9, 13, 37), &PointI16::of(-10, -20)), Rect::of(-5, -11, 3, 17));
    assert_eq!(checked_translate(&Rect::of(-5, -11, 3, 17), &PointI16::of(6, -19)), Rect::of(1, -30, 9, -2));
}

use super::checked_translate;
use crate::cartesian::{point::point_i32::Point, rect::rect_i32::Rect};

#[test]
fn test() {
    assert_eq!(checked_translate(&Rect::of(5, 9, 13, 37), &Point::of(-10, -20)), Rect::of(-5, -11, 3, 17));
    assert_eq!(checked_translate(&Rect::of(-5, -11, 3, 17), &Point::of(6, -19)), Rect::of(1, -30, 9, -2));
}

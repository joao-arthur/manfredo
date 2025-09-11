use super::checked_translate_assign;
use crate::matrix::{point::point_i16::PointI16, rect::rect_i16::Rect};

#[test]
fn test() {
    let mut r = Rect::of(5, 9, 13, 37);
    checked_translate_assign(&mut r, &PointI16::of(-10, -20));
    assert_eq!(r, Rect::of(-5, -11, 3, 17));
    checked_translate_assign(&mut r, &PointI16::of(6, -19));
    assert_eq!(r, Rect::of(1, -30, 9, -2));
}

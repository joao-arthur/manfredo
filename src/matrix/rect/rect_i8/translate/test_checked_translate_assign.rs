use super::checked_translate_assign;
use crate::matrix::{point::point_i8::PointI8, rect::rect_i8::RectI8};

#[test]
fn test() {
    let mut r = RectI8::of(5, 9, 13, 37);
    checked_translate_assign(&mut r, &PointI8::of(-10, -20));
    assert_eq!(r, RectI8::of(-5, -11, 3, 17));
    checked_translate_assign(&mut r, &PointI8::of(6, -19));
    assert_eq!(r, RectI8::of(1, -30, 9, -2));
}

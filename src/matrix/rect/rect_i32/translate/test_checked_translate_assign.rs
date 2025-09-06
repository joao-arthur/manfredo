use super::checked_translate_assign;
use crate::matrix::{point::point_i32::PointI32, rect::rect_i32::RectI32};

#[test]
fn test() {
    let mut r = RectI32::of(5, 9, 13, 37);
    checked_translate_assign(&mut r, &PointI32::of(-10, -20));
    assert_eq!(r, RectI32::of(-5, -11, 3, 17));
    checked_translate_assign(&mut r, &PointI32::of(6, -19));
    assert_eq!(r, RectI32::of(1, -30, 9, -2));
}

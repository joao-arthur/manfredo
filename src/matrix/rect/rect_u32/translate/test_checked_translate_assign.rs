use super::checked_translate_assign;
use crate::matrix::{point::point_i32::PointI32, rect::rect_u32::RectU32};

#[test]
fn test() {
    let mut r = RectU32::of(0, 0, 12, 15);
    checked_translate_assign(&mut r, &PointI32::of(5, 4));
    assert_eq!(r, RectU32::of(5, 4, 17, 19));
    checked_translate_assign(&mut r, &PointI32::of(-4, -2));
    assert_eq!(r, RectU32::of(1, 2, 13, 17));
}

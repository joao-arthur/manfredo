use super::checked_translate_assign;
use crate::matrix::{point::point_i16::PointI16, rect::rect_u16::RectU16};

#[test]
fn test() {
    let mut r = RectU16::of(0, 0, 12, 15);
    checked_translate_assign(&mut r, &PointI16::of(5, 4));
    assert_eq!(r, RectU16::of(5, 4, 17, 19));
    checked_translate_assign(&mut r, &PointI16::of(-4, -2));
    assert_eq!(r, RectU16::of(1, 2, 13, 17));
}

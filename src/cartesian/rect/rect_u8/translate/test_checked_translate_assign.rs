use super::checked_translate_assign;
use crate::cartesian::{point::point_i8::PointI8, rect::rect_u8::RectU8};

#[test]
fn test_checked_translate_assign() {
    let mut r = RectU8::of(0, 0, 12, 15);
    checked_translate_assign(&mut r, &PointI8::of(5, 4));
    assert_eq!(r, RectU8::of(5, 4, 17, 19));
    checked_translate_assign(&mut r, &PointI8::of(-4, -2));
    assert_eq!(r, RectU8::of(1, 2, 13, 17));
}

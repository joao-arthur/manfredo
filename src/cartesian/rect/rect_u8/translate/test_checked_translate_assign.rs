use super::checked_translate_assign;
use crate::cartesian::{point::point_i8::PointI8, rect::rect_u8::Rect};

#[test]
fn test() {
    let mut r = Rect::of(0, 0, 12, 15);
    checked_translate_assign(&mut r, &PointI8::of(5, 4));
    assert_eq!(r, Rect::of(5, 4, 17, 19));
    checked_translate_assign(&mut r, &PointI8::of(-4, -2));
    assert_eq!(r, Rect::of(1, 2, 13, 17));
}

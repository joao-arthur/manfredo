use super::checked_add_assign;
use crate::matrix::rect::{rect_i32::Rect, rect_u32::RectU32};

#[test]
fn test() {
    let mut r = RectU32::of(0, 0, 12, 10);
    checked_add_assign(&mut r, &Rect::of(5, 4, 3, 2));
    assert_eq!(r, RectU32::of(5, 4, 15, 12));
    checked_add_assign(&mut r, &Rect::of(-4, -3, -2, -1));
    assert_eq!(r, RectU32::of(1, 1, 13, 11));
}

use super::checked_add_assign;
use crate::cartesian::rect::rect_i32::RectI32;

#[test]
fn test() {
    let mut r = RectI32::of(-7, 9, -12, 15);
    checked_add_assign(&mut r, &RectI32::of(5, 4, 3, 2));
    assert_eq!(r, RectI32::of(-2, 13, -9, 17));
    checked_add_assign(&mut r, &RectI32::of(9, -10, 11, -12));
    assert_eq!(r, RectI32::of(7, 3, 2, 5));
}

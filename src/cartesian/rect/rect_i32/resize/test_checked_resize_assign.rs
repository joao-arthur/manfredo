use super::checked_resize_assign;
use crate::cartesian::rect::rect_i32::RectI32;

#[test]
fn odd() {
    let mut r = RectI32::of(-5, -5, 5, 5);
    checked_resize_assign(&mut r, 9);
    assert_eq!(r, RectI32::of(-4, -4, 4, 4));
    checked_resize_assign(&mut r, 7);
    assert_eq!(r, RectI32::of(-3, -3, 3, 3));
    checked_resize_assign(&mut r, 5);
    assert_eq!(r, RectI32::of(-2, -2, 2, 2));
    checked_resize_assign(&mut r, 3);
    assert_eq!(r, RectI32::of(-1, -1, 1, 1));
    checked_resize_assign(&mut r, 9);
    assert_eq!(r, RectI32::of(-4, -4, 4, 4));
}

#[test]
fn even() {
    let mut r = RectI32::of(-5, -5, 4, 4);
    checked_resize_assign(&mut r, 10);
    assert_eq!(r, RectI32::of(-5, -5, 4, 4));
    checked_resize_assign(&mut r, 8);
    assert_eq!(r, RectI32::of(-4, -4, 3, 3));
    checked_resize_assign(&mut r, 6);
    assert_eq!(r, RectI32::of(-3, -3, 2, 2));
    checked_resize_assign(&mut r, 4);
    assert_eq!(r, RectI32::of(-2, -2, 1, 1));
    checked_resize_assign(&mut r, 8);
    assert_eq!(r, RectI32::of(-4, -4, 3, 3));
}

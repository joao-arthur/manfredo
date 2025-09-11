use super::saturating_resize_assign;
use crate::matrix::rect::rect_i8::Rect;

#[test]
fn odd() {
    let mut r = Rect::of(-5, -5, 5, 5);
    saturating_resize_assign(&mut r, 9);
    assert_eq!(r, Rect::of(-4, -4, 4, 4));
    saturating_resize_assign(&mut r, 7);
    assert_eq!(r, Rect::of(-3, -3, 3, 3));
    saturating_resize_assign(&mut r, 5);
    assert_eq!(r, Rect::of(-2, -2, 2, 2));
    saturating_resize_assign(&mut r, 3);
    assert_eq!(r, Rect::of(-1, -1, 1, 1));
    saturating_resize_assign(&mut r, 9);
    assert_eq!(r, Rect::of(-4, -4, 4, 4));
}

#[test]
fn even() {
    let mut r = Rect::of(-5, -5, 4, 4);
    saturating_resize_assign(&mut r, 10);
    assert_eq!(r, Rect::of(-5, -5, 4, 4));
    saturating_resize_assign(&mut r, 8);
    assert_eq!(r, Rect::of(-4, -4, 3, 3));
    saturating_resize_assign(&mut r, 6);
    assert_eq!(r, Rect::of(-3, -3, 2, 2));
    saturating_resize_assign(&mut r, 4);
    assert_eq!(r, Rect::of(-2, -2, 1, 1));
    saturating_resize_assign(&mut r, 8);
    assert_eq!(r, Rect::of(-4, -4, 3, 3));
}

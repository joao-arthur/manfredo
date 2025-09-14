use super::checked_resize_assign;
use crate::cartesian::rect::rect_f64::Rect;

#[test]
fn odd() {
    let mut r = Rect::of(-5.0, -5.0, 5.0, 5.0);
    checked_resize_assign(&mut r, 9.0);
    assert_eq!(r, Rect::of(-4.0, -4.0, 4.0, 4.0));
    checked_resize_assign(&mut r, 7.0);
    assert_eq!(r, Rect::of(-3.0, -3.0, 3.0, 3.0));
    checked_resize_assign(&mut r, 5.0);
    assert_eq!(r, Rect::of(-2.0, -2.0, 2.0, 2.0));
    checked_resize_assign(&mut r, 3.0);
    assert_eq!(r, Rect::of(-1.0, -1.0, 1.0, 1.0));
    checked_resize_assign(&mut r, 9.0);
    assert_eq!(r, Rect::of(-4.0, -4.0, 4.0, 4.0));
}

#[test]
fn even() {
    let mut r = Rect::of(-5.0, -5.0, 4.0, 4.0);
    checked_resize_assign(&mut r, 10.0);
    assert_eq!(r, Rect::of(-5.0, -5.0, 4.0, 4.0));
    checked_resize_assign(&mut r, 8.0);
    assert_eq!(r, Rect::of(-4.0, -4.0, 3.0, 3.0));
    checked_resize_assign(&mut r, 6.0);
    assert_eq!(r, Rect::of(-3.0, -3.0, 2.0, 2.0));
    checked_resize_assign(&mut r, 4.0);
    assert_eq!(r, Rect::of(-2.0, -2.0, 1.0, 1.0));
    checked_resize_assign(&mut r, 8.0);
    assert_eq!(r, Rect::of(-4.0, -4.0, 3.0, 3.0));
}

use super::wrapping_resize_assign;
use crate::matrix::d2::rect::rect_i16::Rect;

#[test]
fn odd() {
    let mut r = Rect::new((-5, -5), (5, 5));
    wrapping_resize_assign(&mut r, 9);
    assert_eq!(r, Rect::new((-4, -4), (4, 4)));
    wrapping_resize_assign(&mut r, 7);
    assert_eq!(r, Rect::new((-3, -3), (3, 3)));
    wrapping_resize_assign(&mut r, 5);
    assert_eq!(r, Rect::new((-2, -2), (2, 2)));
    wrapping_resize_assign(&mut r, 3);
    assert_eq!(r, Rect::new((-1, -1), (1, 1)));
    wrapping_resize_assign(&mut r, 9);
    assert_eq!(r, Rect::new((-4, -4), (4, 4)));
}

#[test]
fn even() {
    let mut r = Rect::new((-5, -5), (4, 4));
    wrapping_resize_assign(&mut r, 10);
    assert_eq!(r, Rect::new((-5, -5), (4, 4)));
    wrapping_resize_assign(&mut r, 8);
    assert_eq!(r, Rect::new((-4, -4), (3, 3)));
    wrapping_resize_assign(&mut r, 6);
    assert_eq!(r, Rect::new((-3, -3), (2, 2)));
    wrapping_resize_assign(&mut r, 4);
    assert_eq!(r, Rect::new((-2, -2), (1, 1)));
    wrapping_resize_assign(&mut r, 8);
    assert_eq!(r, Rect::new((-4, -4), (3, 3)));
}

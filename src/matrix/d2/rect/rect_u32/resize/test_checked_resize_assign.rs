use super::checked_resize_assign;
use crate::matrix::rect::rect_u32::Rect;

#[test]
fn odd() {
    let mut r = Rect::of(5, 5, 15, 15);
    checked_resize_assign(&mut r, 9);
    assert_eq!(r, Rect::of(6, 6, 14, 14));
    checked_resize_assign(&mut r, 7);
    assert_eq!(r, Rect::of(7, 7, 13, 13));
    checked_resize_assign(&mut r, 5);
    assert_eq!(r, Rect::of(8, 8, 12, 12));
    checked_resize_assign(&mut r, 3);
    assert_eq!(r, Rect::of(9, 9, 11, 11));
    checked_resize_assign(&mut r, 9);
    assert_eq!(r, Rect::of(6, 6, 14, 14));
}

#[test]
fn even() {
    let mut r = Rect::of(5, 5, 14, 14);
    checked_resize_assign(&mut r, 10);
    assert_eq!(r, Rect::of(5, 5, 14, 14));
    checked_resize_assign(&mut r, 8);
    assert_eq!(r, Rect::of(6, 6, 13, 13));
    checked_resize_assign(&mut r, 6);
    assert_eq!(r, Rect::of(7, 7, 12, 12));
    checked_resize_assign(&mut r, 4);
    assert_eq!(r, Rect::of(8, 8, 11, 11));
    checked_resize_assign(&mut r, 8);
    assert_eq!(r, Rect::of(6, 6, 13, 13));
}

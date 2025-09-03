use super::wrapping_resize_assign;
use crate::cartesian::rect::rect_u16::RectU16;

#[test]
fn wrapping_resize_assign_odd() {
    let mut r = RectU16::of(5, 5, 15, 15);
    wrapping_resize_assign(&mut r, 9);
    assert_eq!(r, RectU16::of(6, 6, 14, 14));
    wrapping_resize_assign(&mut r, 7);
    assert_eq!(r, RectU16::of(7, 7, 13, 13));
    wrapping_resize_assign(&mut r, 5);
    assert_eq!(r, RectU16::of(8, 8, 12, 12));
    wrapping_resize_assign(&mut r, 3);
    assert_eq!(r, RectU16::of(9, 9, 11, 11));
    wrapping_resize_assign(&mut r, 9);
    assert_eq!(r, RectU16::of(6, 6, 14, 14));
}

#[test]
fn wrapping_resize_assign_even() {
    let mut r = RectU16::of(5, 5, 14, 14);
    wrapping_resize_assign(&mut r, 10);
    assert_eq!(r, RectU16::of(5, 5, 14, 14));
    wrapping_resize_assign(&mut r, 8);
    assert_eq!(r, RectU16::of(6, 6, 13, 13));
    wrapping_resize_assign(&mut r, 6);
    assert_eq!(r, RectU16::of(7, 7, 12, 12));
    wrapping_resize_assign(&mut r, 4);
    assert_eq!(r, RectU16::of(8, 8, 11, 11));
    wrapping_resize_assign(&mut r, 8);
    assert_eq!(r, RectU16::of(6, 6, 13, 13));
}

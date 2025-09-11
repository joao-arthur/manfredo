use super::wrapping_resize;
use crate::cartesian::rect::rect_u8::Rect;

#[test]
fn odd() {
    assert_eq!(wrapping_resize(&Rect::of(5, 5, 15, 15), 9), Rect::of(6, 6, 14, 14));
    assert_eq!(wrapping_resize(&Rect::of(6, 6, 14, 14), 7), Rect::of(7, 7, 13, 13));
    assert_eq!(wrapping_resize(&Rect::of(7, 7, 13, 13), 5), Rect::of(8, 8, 12, 12));
    assert_eq!(wrapping_resize(&Rect::of(8, 8, 12, 12), 3), Rect::of(9, 9, 11, 11));
    assert_eq!(wrapping_resize(&Rect::of(9, 9, 11, 11), 9), Rect::of(6, 6, 14, 14));
}

#[test]
fn even() {
    assert_eq!(wrapping_resize(&Rect::of(5, 5, 14, 14), 10), Rect::of(5, 5, 14, 14));
    assert_eq!(wrapping_resize(&Rect::of(5, 5, 14, 14), 8), Rect::of(6, 6, 13, 13));
    assert_eq!(wrapping_resize(&Rect::of(6, 6, 13, 13), 6), Rect::of(7, 7, 12, 12));
    assert_eq!(wrapping_resize(&Rect::of(7, 7, 12, 12), 4), Rect::of(8, 8, 11, 11));
    assert_eq!(wrapping_resize(&Rect::of(8, 8, 11, 11), 8), Rect::of(6, 6, 13, 13));
}

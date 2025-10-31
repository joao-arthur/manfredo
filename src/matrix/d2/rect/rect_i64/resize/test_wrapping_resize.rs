use super::wrapping_resize;
use crate::matrix::d2::rect::rect_i64::Rect;

#[test]
fn odd() {
    assert_eq!(wrapping_resize(&Rect::of((-5, -5), (5, 5)), 9), Rect::of((-4, -4), (4, 4)));
    assert_eq!(wrapping_resize(&Rect::of((-4, -4), (4, 4)), 7), Rect::of((-3, -3), (3, 3)));
    assert_eq!(wrapping_resize(&Rect::of((-3, -3), (3, 3)), 5), Rect::of((-2, -2), (2, 2)));
    assert_eq!(wrapping_resize(&Rect::of((-2, -2), (2, 2)), 3), Rect::of((-1, -1), (1, 1)));
    assert_eq!(wrapping_resize(&Rect::of((-1, -1), (1, 1)), 9), Rect::of((-4, -4), (4, 4)));
}

#[test]
fn even() {
    assert_eq!(wrapping_resize(&Rect::of((-5, -5), (4, 4)), 10), Rect::of((-5, -5), (4, 4)));
    assert_eq!(wrapping_resize(&Rect::of((-5, -5), (4, 4)), 8), Rect::of((-4, -4), (3, 3)));
    assert_eq!(wrapping_resize(&Rect::of((-4, -4), (3, 3)), 6), Rect::of((-3, -3), (2, 2)));
    assert_eq!(wrapping_resize(&Rect::of((-3, -3), (2, 2)), 4), Rect::of((-2, -2), (1, 1)));
    assert_eq!(wrapping_resize(&Rect::of((-2, -2), (1, 1)), 8), Rect::of((-4, -4), (3, 3)));
}

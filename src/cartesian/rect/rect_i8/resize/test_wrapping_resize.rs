use super::wrapping_resize;
use crate::cartesian::rect::rect_i8::RectI8;

#[test]
fn odd() {
    assert_eq!(wrapping_resize(&RectI8::of(-5, -5, 5, 5), 9), RectI8::of(-4, -4, 4, 4));
    assert_eq!(wrapping_resize(&RectI8::of(-4, -4, 4, 4), 7), RectI8::of(-3, -3, 3, 3));
    assert_eq!(wrapping_resize(&RectI8::of(-3, -3, 3, 3), 5), RectI8::of(-2, -2, 2, 2));
    assert_eq!(wrapping_resize(&RectI8::of(-2, -2, 2, 2), 3), RectI8::of(-1, -1, 1, 1));
    assert_eq!(wrapping_resize(&RectI8::of(-1, -1, 1, 1), 9), RectI8::of(-4, -4, 4, 4));
}

#[test]
fn even() {
    assert_eq!(wrapping_resize(&RectI8::of(-5, -5, 4, 4), 10), RectI8::of(-5, -5, 4, 4));
    assert_eq!(wrapping_resize(&RectI8::of(-5, -5, 4, 4), 8), RectI8::of(-4, -4, 3, 3));
    assert_eq!(wrapping_resize(&RectI8::of(-4, -4, 3, 3), 6), RectI8::of(-3, -3, 2, 2));
    assert_eq!(wrapping_resize(&RectI8::of(-3, -3, 2, 2), 4), RectI8::of(-2, -2, 1, 1));
    assert_eq!(wrapping_resize(&RectI8::of(-2, -2, 1, 1), 8), RectI8::of(-4, -4, 3, 3));
}

use super::saturating_resize;
use crate::cartesian::rect::rect_i32::RectI32;

#[test]
fn odd() {
    assert_eq!(saturating_resize(&RectI32::of(-5, -5, 5, 5), 9), RectI32::of(-4, -4, 4, 4));
    assert_eq!(saturating_resize(&RectI32::of(-4, -4, 4, 4), 7), RectI32::of(-3, -3, 3, 3));
    assert_eq!(saturating_resize(&RectI32::of(-3, -3, 3, 3), 5), RectI32::of(-2, -2, 2, 2));
    assert_eq!(saturating_resize(&RectI32::of(-2, -2, 2, 2), 3), RectI32::of(-1, -1, 1, 1));
    assert_eq!(saturating_resize(&RectI32::of(-1, -1, 1, 1), 9), RectI32::of(-4, -4, 4, 4));
}

#[test]
fn even() {
    assert_eq!(saturating_resize(&RectI32::of(-5, -5, 4, 4), 10), RectI32::of(-5, -5, 4, 4));
    assert_eq!(saturating_resize(&RectI32::of(-5, -5, 4, 4), 8), RectI32::of(-4, -4, 3, 3));
    assert_eq!(saturating_resize(&RectI32::of(-4, -4, 3, 3), 6), RectI32::of(-3, -3, 2, 2));
    assert_eq!(saturating_resize(&RectI32::of(-3, -3, 2, 2), 4), RectI32::of(-2, -2, 1, 1));
    assert_eq!(saturating_resize(&RectI32::of(-2, -2, 1, 1), 8), RectI32::of(-4, -4, 3, 3));
}

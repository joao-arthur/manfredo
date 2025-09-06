use super::checked_resize;
use crate::cartesian::rect::rect_i64::RectI64;

#[test]
fn odd() {
    assert_eq!(checked_resize(&RectI64::of(-5, -5, 5, 5), 9), RectI64::of(-4, -4, 4, 4));
    assert_eq!(checked_resize(&RectI64::of(-4, -4, 4, 4), 7), RectI64::of(-3, -3, 3, 3));
    assert_eq!(checked_resize(&RectI64::of(-3, -3, 3, 3), 5), RectI64::of(-2, -2, 2, 2));
    assert_eq!(checked_resize(&RectI64::of(-2, -2, 2, 2), 3), RectI64::of(-1, -1, 1, 1));
    assert_eq!(checked_resize(&RectI64::of(-1, -1, 1, 1), 9), RectI64::of(-4, -4, 4, 4));
}

#[test]
fn even() {
    assert_eq!(checked_resize(&RectI64::of(-5, -5, 4, 4), 10), RectI64::of(-5, -5, 4, 4));
    assert_eq!(checked_resize(&RectI64::of(-5, -5, 4, 4), 8), RectI64::of(-4, -4, 3, 3));
    assert_eq!(checked_resize(&RectI64::of(-4, -4, 3, 3), 6), RectI64::of(-3, -3, 2, 2));
    assert_eq!(checked_resize(&RectI64::of(-3, -3, 2, 2), 4), RectI64::of(-2, -2, 1, 1));
    assert_eq!(checked_resize(&RectI64::of(-2, -2, 1, 1), 8), RectI64::of(-4, -4, 3, 3));
}

use super::saturating_resize;
use crate::cartesian::rect::rect_f64::RectF64;

#[test]
fn odd() {
    assert_eq!(saturating_resize(&RectF64::of(-5.0, -5.0, 5.0, 5.0), 9.0), RectF64::of(-4.0, -4.0, 4.0, 4.0));
    assert_eq!(saturating_resize(&RectF64::of(-4.0, -4.0, 4.0, 4.0), 7.0), RectF64::of(-3.0, -3.0, 3.0, 3.0));
    assert_eq!(saturating_resize(&RectF64::of(-3.0, -3.0, 3.0, 3.0), 5.0), RectF64::of(-2.0, -2.0, 2.0, 2.0));
    assert_eq!(saturating_resize(&RectF64::of(-2.0, -2.0, 2.0, 2.0), 3.0), RectF64::of(-1.0, -1.0, 1.0, 1.0));
    assert_eq!(saturating_resize(&RectF64::of(-1.0, -1.0, 1.0, 1.0), 9.0), RectF64::of(-4.0, -4.0, 4.0, 4.0));
}

#[test]
fn even() {
    assert_eq!(saturating_resize(&RectF64::of(-5.0, -5.0, 4.0, 4.0), 10.0), RectF64::of(-5.0, -5.0, 4.0, 4.0));
    assert_eq!(saturating_resize(&RectF64::of(-5.0, -5.0, 4.0, 4.0), 8.0), RectF64::of(-4.0, -4.0, 3.0, 3.0));
    assert_eq!(saturating_resize(&RectF64::of(-4.0, -4.0, 3.0, 3.0), 6.0), RectF64::of(-3.0, -3.0, 2.0, 2.0));
    assert_eq!(saturating_resize(&RectF64::of(-3.0, -3.0, 2.0, 2.0), 4.0), RectF64::of(-2.0, -2.0, 1.0, 1.0));
    assert_eq!(saturating_resize(&RectF64::of(-2.0, -2.0, 1.0, 1.0), 8.0), RectF64::of(-4.0, -4.0, 3.0, 3.0));
}

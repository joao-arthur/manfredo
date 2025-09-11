use super::saturating_resize;
use crate::cartesian::rect::rect_f32::Rect;

#[test]
fn odd() {
    assert_eq!(saturating_resize(&Rect::of(-5.0, -5.0, 5.0, 5.0), 9.0), Rect::of(-4.0, -4.0, 4.0, 4.0));
    assert_eq!(saturating_resize(&Rect::of(-4.0, -4.0, 4.0, 4.0), 7.0), Rect::of(-3.0, -3.0, 3.0, 3.0));
    assert_eq!(saturating_resize(&Rect::of(-3.0, -3.0, 3.0, 3.0), 5.0), Rect::of(-2.0, -2.0, 2.0, 2.0));
    assert_eq!(saturating_resize(&Rect::of(-2.0, -2.0, 2.0, 2.0), 3.0), Rect::of(-1.0, -1.0, 1.0, 1.0));
    assert_eq!(saturating_resize(&Rect::of(-1.0, -1.0, 1.0, 1.0), 9.0), Rect::of(-4.0, -4.0, 4.0, 4.0));
}

#[test]
fn even() {
    assert_eq!(saturating_resize(&Rect::of(-5.0, -5.0, 4.0, 4.0), 10.0), Rect::of(-5.0, -5.0, 4.0, 4.0));
    assert_eq!(saturating_resize(&Rect::of(-5.0, -5.0, 4.0, 4.0), 8.0), Rect::of(-4.0, -4.0, 3.0, 3.0));
    assert_eq!(saturating_resize(&Rect::of(-4.0, -4.0, 3.0, 3.0), 6.0), Rect::of(-3.0, -3.0, 2.0, 2.0));
    assert_eq!(saturating_resize(&Rect::of(-3.0, -3.0, 2.0, 2.0), 4.0), Rect::of(-2.0, -2.0, 1.0, 1.0));
    assert_eq!(saturating_resize(&Rect::of(-2.0, -2.0, 1.0, 1.0), 8.0), Rect::of(-4.0, -4.0, 3.0, 3.0));
}

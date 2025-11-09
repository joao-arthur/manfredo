use super::saturating_resize;
use crate::matrix::d2::rect::rect_i64::Rect;

#[test]
fn odd() {
    assert_eq!(saturating_resize(&Rect::new((-5, -5), (5, 5)), 9), Rect::new((-4, -4), (4, 4)));
    assert_eq!(saturating_resize(&Rect::new((-4, -4), (4, 4)), 7), Rect::new((-3, -3), (3, 3)));
    assert_eq!(saturating_resize(&Rect::new((-3, -3), (3, 3)), 5), Rect::new((-2, -2), (2, 2)));
    assert_eq!(saturating_resize(&Rect::new((-2, -2), (2, 2)), 3), Rect::new((-1, -1), (1, 1)));
    assert_eq!(saturating_resize(&Rect::new((-1, -1), (1, 1)), 9), Rect::new((-4, -4), (4, 4)));
}

#[test]
fn even() {
    assert_eq!(saturating_resize(&Rect::new((-5, -5), (4, 4)), 10), Rect::new((-5, -5), (4, 4)));
    assert_eq!(saturating_resize(&Rect::new((-5, -5), (4, 4)), 8), Rect::new((-4, -4), (3, 3)));
    assert_eq!(saturating_resize(&Rect::new((-4, -4), (3, 3)), 6), Rect::new((-3, -3), (2, 2)));
    assert_eq!(saturating_resize(&Rect::new((-3, -3), (2, 2)), 4), Rect::new((-2, -2), (1, 1)));
    assert_eq!(saturating_resize(&Rect::new((-2, -2), (1, 1)), 8), Rect::new((-4, -4), (3, 3)));
}

use super::checked_resize;
use crate::cartesian::d2::rect::rect_f64::Rect;

#[test]
fn odd() {
    assert_eq!(checked_resize(&Rect::new((-5.0, -5.0), (5.0, 5.0)), 9.0), Rect::new((-4.0, -4.0), (4.0, 4.0)));
    assert_eq!(checked_resize(&Rect::new((-4.0, -4.0), (4.0, 4.0)), 7.0), Rect::new((-3.0, -3.0), (3.0, 3.0)));
    assert_eq!(checked_resize(&Rect::new((-3.0, -3.0), (3.0, 3.0)), 5.0), Rect::new((-2.0, -2.0), (2.0, 2.0)));
    assert_eq!(checked_resize(&Rect::new((-2.0, -2.0), (2.0, 2.0)), 3.0), Rect::new((-1.0, -1.0), (1.0, 1.0)));
    assert_eq!(checked_resize(&Rect::new((-1.0, -1.0), (1.0, 1.0)), 9.0), Rect::new((-4.0, -4.0), (4.0, 4.0)));
}

#[test]
fn even() {
    assert_eq!(checked_resize(&Rect::new((-5.0, -5.0), (4.0, 4.0)), 10.0), Rect::new((-5.0, -5.0), (4.0, 4.0)));
    assert_eq!(checked_resize(&Rect::new((-5.0, -5.0), (4.0, 4.0)), 8.0), Rect::new((-4.0, -4.0), (3.0, 3.0)));
    assert_eq!(checked_resize(&Rect::new((-4.0, -4.0), (3.0, 3.0)), 6.0), Rect::new((-3.0, -3.0), (2.0, 2.0)));
    assert_eq!(checked_resize(&Rect::new((-3.0, -3.0), (2.0, 2.0)), 4.0), Rect::new((-2.0, -2.0), (1.0, 1.0)));
    assert_eq!(checked_resize(&Rect::new((-2.0, -2.0), (1.0, 1.0)), 8.0), Rect::new((-4.0, -4.0), (3.0, 3.0)));
}

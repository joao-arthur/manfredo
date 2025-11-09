use super::try_saturating_resize;
use crate::cartesian::{
    d1::point::point_f32::{MAX, MIN},
    d2::rect::rect_f32::Rect,
};

#[test]
fn odd() {
    assert_eq!(try_saturating_resize(&Rect::new((-5.0, -5.0), (5.0, 5.0)), 9.0), Some(Rect::new((-4.0, -4.0), (4.0, 4.0))));
    assert_eq!(try_saturating_resize(&Rect::new((-4.0, -4.0), (4.0, 4.0)), 7.0), Some(Rect::new((-3.0, -3.0), (3.0, 3.0))));
    assert_eq!(try_saturating_resize(&Rect::new((-3.0, -3.0), (3.0, 3.0)), 5.0), Some(Rect::new((-2.0, -2.0), (2.0, 2.0))));
    assert_eq!(try_saturating_resize(&Rect::new((-2.0, -2.0), (2.0, 2.0)), 3.0), Some(Rect::new((-1.0, -1.0), (1.0, 1.0))));
    assert_eq!(try_saturating_resize(&Rect::new((-1.0, -1.0), (1.0, 1.0)), 9.0), Some(Rect::new((-4.0, -4.0), (4.0, 4.0))));
}

#[test]
fn even() {
    assert_eq!(try_saturating_resize(&Rect::new((-5.0, -5.0), (4.0, 4.0)), 10.0), Some(Rect::new((-5.0, -5.0), (4.0, 4.0))));
    assert_eq!(try_saturating_resize(&Rect::new((-5.0, -5.0), (4.0, 4.0)), 8.0), Some(Rect::new((-4.0, -4.0), (3.0, 3.0))));
    assert_eq!(try_saturating_resize(&Rect::new((-4.0, -4.0), (3.0, 3.0)), 6.0), Some(Rect::new((-3.0, -3.0), (2.0, 2.0))));
    assert_eq!(try_saturating_resize(&Rect::new((-3.0, -3.0), (2.0, 2.0)), 4.0), Some(Rect::new((-2.0, -2.0), (1.0, 1.0))));
    assert_eq!(try_saturating_resize(&Rect::new((-2.0, -2.0), (1.0, 1.0)), 8.0), Some(Rect::new((-4.0, -4.0), (3.0, 3.0))));
}

#[test]
fn small_size() {
    let r = Rect::new((10.0, 10.0), (20.0, 20.0));
    assert_eq!(try_saturating_resize(&r, 0.0), None);
    assert_eq!(try_saturating_resize(&r, 1.0), None);
    assert_eq!(try_saturating_resize(&r, 2.0), None);
    assert_eq!(try_saturating_resize(&r, MAX + 1.0), None);
    assert_eq!(try_saturating_resize(&r, MAX + 2.0), None);
    assert_eq!(try_saturating_resize(&r, MAX + 3.0), None);
}

#[test]
fn same_size() {
    assert_eq!(try_saturating_resize(&Rect::new((MIN, MIN), (MIN + 2.0, MIN + 2.0)), 3.0), Some(Rect::new((MIN, MIN), (MIN + 2.0, MIN + 2.0))));
    assert_eq!(try_saturating_resize(&Rect::new((MIN, MIN), (MIN + 3.0, MIN + 3.0)), 4.0), Some(Rect::new((MIN, MIN), (MIN + 3.0, MIN + 3.0))));
    assert_eq!(try_saturating_resize(&Rect::new((MAX - 2.0, MAX - 2.0), (MAX, MAX)), 3.0), Some(Rect::new((MAX - 2.0, MAX - 2.0), (MAX, MAX))));
    assert_eq!(try_saturating_resize(&Rect::new((MAX - 3.0, MAX - 3.0), (MAX, MAX)), 4.0), Some(Rect::new((MAX - 3.0, MAX - 3.0), (MAX, MAX))));
}

#[test]
fn bounds() {
    assert_eq!(try_saturating_resize(&Rect::new((MIN, MIN), (MIN + 2.0, MIN + 2.0)), 11.0), Some(Rect::new((MIN, MIN), (MIN + 10.0, MIN + 10.0))));
    assert_eq!(try_saturating_resize(&Rect::new((MAX - 2.0, MAX - 2.0), (MAX, MAX)), 11.0), Some(Rect::new((MAX - 10.0, MAX - 10.0), (MAX, MAX))));
}

#[test]
fn small_rect_limits() {
    assert_eq!(try_saturating_resize(&Rect::new((MIN, MIN), (MIN + 2.0, MIN + 2.0)), MAX), Some(Rect::new((MIN, MIN), (-2.0, -2.0))));
    assert_eq!(try_saturating_resize(&Rect::new((MAX - 2.0, MAX - 2.0), (MAX, MAX)), MAX), Some(Rect::new((1.0, 1.0), (MAX, MAX))));
}

#[test]
fn big_rect_limits() {
    assert_eq!(try_saturating_resize(&Rect::new((MIN, MIN), (-2.0, -2.0)), MAX), Some(Rect::new((MIN, MIN), (-2.0, -2.0))));
    assert_eq!(try_saturating_resize(&Rect::new((MIN + 1.0, MIN + 1.0), (-1.0, -1.0)), MAX), Some(Rect::new((MIN + 1.0, MIN + 1.0), (-1.0, -1.0))));
    assert_eq!(try_saturating_resize(&Rect::new((MIN + 2.0, MIN + 2.0), (0.0, 0.0)), MAX), Some(Rect::new((MIN + 2.0, MIN + 2.0), (0.0, 0.0))));
    assert_eq!(try_saturating_resize(&Rect::new((0.0, 0.0), (MAX - 1.0, MAX - 1.0)), MAX), Some(Rect::new((0.0, 0.0), (MAX - 1.0, MAX - 1.0))));
    assert_eq!(try_saturating_resize(&Rect::new((1.0, 1.0), (MAX, MAX)), MAX), Some(Rect::new((1.0, 1.0), (MAX, MAX))));
}

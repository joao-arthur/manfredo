use super::try_checked_resize;
use crate::cartesian::{
    point::point_f64::{MAX, MIN},
    rect::rect_f64::Rect,
};

#[test]
fn odd() {
    assert_eq!(try_checked_resize(&Rect::of(-5.0, -5.0, 5.0, 5.0), 9.0), Some(Rect::of(-4.0, -4.0, 4.0, 4.0)));
    assert_eq!(try_checked_resize(&Rect::of(-4.0, -4.0, 4.0, 4.0), 7.0), Some(Rect::of(-3.0, -3.0, 3.0, 3.0)));
    assert_eq!(try_checked_resize(&Rect::of(-3.0, -3.0, 3.0, 3.0), 5.0), Some(Rect::of(-2.0, -2.0, 2.0, 2.0)));
    assert_eq!(try_checked_resize(&Rect::of(-2.0, -2.0, 2.0, 2.0), 3.0), Some(Rect::of(-1.0, -1.0, 1.0, 1.0)));
    assert_eq!(try_checked_resize(&Rect::of(-1.0, -1.0, 1.0, 1.0), 9.0), Some(Rect::of(-4.0, -4.0, 4.0, 4.0)));
}

#[test]
fn even() {
    assert_eq!(try_checked_resize(&Rect::of(-5.0, -5.0, 4.0, 4.0), 10.0), Some(Rect::of(-5.0, -5.0, 4.0, 4.0)));
    assert_eq!(try_checked_resize(&Rect::of(-5.0, -5.0, 4.0, 4.0), 8.0), Some(Rect::of(-4.0, -4.0, 3.0, 3.0)));
    assert_eq!(try_checked_resize(&Rect::of(-4.0, -4.0, 3.0, 3.0), 6.0), Some(Rect::of(-3.0, -3.0, 2.0, 2.0)));
    assert_eq!(try_checked_resize(&Rect::of(-3.0, -3.0, 2.0, 2.0), 4.0), Some(Rect::of(-2.0, -2.0, 1.0, 1.0)));
    assert_eq!(try_checked_resize(&Rect::of(-2.0, -2.0, 1.0, 1.0), 8.0), Some(Rect::of(-4.0, -4.0, 3.0, 3.0)));
}

#[test]
fn small_size() {
    let r = Rect::of(10.0, 10.0, 20.0, 20.0);
    assert_eq!(try_checked_resize(&r, 0.0), None);
    assert_eq!(try_checked_resize(&r, 1.0), None);
    assert_eq!(try_checked_resize(&r, 2.0), None);
    assert_eq!(try_checked_resize(&r, MAX + 1.0), None);
    assert_eq!(try_checked_resize(&r, MAX + 2.0), None);
    assert_eq!(try_checked_resize(&r, MAX + 3.0), None);
}

#[test]
fn same_size() {
    assert_eq!(try_checked_resize(&Rect::of(MIN, MIN, MIN + 2.0, MIN + 2.0), 3.0), Some(Rect::of(MIN, MIN, MIN + 2.0, MIN + 2.0)));
    assert_eq!(try_checked_resize(&Rect::of(MIN, MIN, MIN + 3.0, MIN + 3.0), 4.0), Some(Rect::of(MIN, MIN, MIN + 3.0, MIN + 3.0)));
    assert_eq!(try_checked_resize(&Rect::of(MAX - 2.0, MAX - 2.0, MAX, MAX), 3.0), Some(Rect::of(MAX - 2.0, MAX - 2.0, MAX, MAX)));
    assert_eq!(try_checked_resize(&Rect::of(MAX - 3.0, MAX - 3.0, MAX, MAX), 4.0), Some(Rect::of(MAX - 3.0, MAX - 3.0, MAX, MAX)));
}

#[test]
fn out_of_bounds() {
    assert_eq!(try_checked_resize(&Rect::of(MIN, MIN + 2.0, MIN + 2.0, MIN + 4.0), 5.0), None);
    assert_eq!(try_checked_resize(&Rect::of(MIN + 2.0, MIN, MIN + 4.0, MIN + 2.0), 5.0), None);
    assert_eq!(try_checked_resize(&Rect::of(MAX - 2.0, MAX - 4.0, MAX, MAX - 2.0), 5.0), None);
    assert_eq!(try_checked_resize(&Rect::of(MAX - 4.0, MAX - 2.0, MAX - 2.0, MAX), 5.0), None);
}

#[test]
fn small_rect_limits_out_of_bounds() {
    assert_eq!(try_checked_resize(&Rect::of(MIN, MIN + 2.0, MIN + 2.0, MIN + 4.0), MAX), None);
    assert_eq!(try_checked_resize(&Rect::of(MIN + 2.0, MIN, MIN + 4.0, MIN + 2.0), MAX), None);
    assert_eq!(try_checked_resize(&Rect::of(MAX - 2.0, MAX - 4.0, MAX, MAX - 2.0), MAX), None);
    assert_eq!(try_checked_resize(&Rect::of(MAX - 4.0, MAX - 2.0, MAX - 2.0, MAX), MAX), None);
}

#[test]
fn big_rect_limits_out_of_bounds() {
    assert_eq!(try_checked_resize(&Rect::of(MIN, MIN, -2.0, -2.0), MAX), Some(Rect::of(MIN, MIN, -2.0, -2.0)));
    assert_eq!(try_checked_resize(&Rect::of(MIN + 1.0, MIN + 1.0, -1.0, -1.0), MAX), Some(Rect::of(MIN + 1.0, MIN + 1.0, -1.0, -1.0)));
    assert_eq!(try_checked_resize(&Rect::of(MIN + 2.0, MIN + 2.0, 0.0, 0.0), MAX), Some(Rect::of(MIN + 2.0, MIN + 2.0, 0.0, 0.0)));
    assert_eq!(try_checked_resize(&Rect::of(0.0, 0.0, MAX - 1.0, MAX - 1.0), MAX), Some(Rect::of(0.0, 0.0, MAX - 1.0, MAX - 1.0)));
    assert_eq!(try_checked_resize(&Rect::of(1.0, 1.0, MAX, MAX), MAX), Some(Rect::of(1.0, 1.0, MAX, MAX)));
}

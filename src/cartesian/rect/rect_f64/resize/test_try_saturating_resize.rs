use super::try_saturating_resize;
use crate::cartesian::{
    point::point_f64::{MAX, MIN},
    rect::rect_f64::RectF64,
};

#[test]
fn odd() {
    assert_eq!(try_saturating_resize(&RectF64::of(-5.0, -5.0, 5.0, 5.0), 9.0), Some(RectF64::of(-4.0, -4.0, 4.0, 4.0)));
    assert_eq!(try_saturating_resize(&RectF64::of(-4.0, -4.0, 4.0, 4.0), 7.0), Some(RectF64::of(-3.0, -3.0, 3.0, 3.0)));
    assert_eq!(try_saturating_resize(&RectF64::of(-3.0, -3.0, 3.0, 3.0), 5.0), Some(RectF64::of(-2.0, -2.0, 2.0, 2.0)));
    assert_eq!(try_saturating_resize(&RectF64::of(-2.0, -2.0, 2.0, 2.0), 3.0), Some(RectF64::of(-1.0, -1.0, 1.0, 1.0)));
    assert_eq!(try_saturating_resize(&RectF64::of(-1.0, -1.0, 1.0, 1.0), 9.0), Some(RectF64::of(-4.0, -4.0, 4.0, 4.0)));
}

#[test]
fn even() {
    assert_eq!(try_saturating_resize(&RectF64::of(-5.0, -5.0, 4.0, 4.0), 10.0), Some(RectF64::of(-5.0, -5.0, 4.0, 4.0)));
    assert_eq!(try_saturating_resize(&RectF64::of(-5.0, -5.0, 4.0, 4.0), 8.0), Some(RectF64::of(-4.0, -4.0, 3.0, 3.0)));
    assert_eq!(try_saturating_resize(&RectF64::of(-4.0, -4.0, 3.0, 3.0), 6.0), Some(RectF64::of(-3.0, -3.0, 2.0, 2.0)));
    assert_eq!(try_saturating_resize(&RectF64::of(-3.0, -3.0, 2.0, 2.0), 4.0), Some(RectF64::of(-2.0, -2.0, 1.0, 1.0)));
    assert_eq!(try_saturating_resize(&RectF64::of(-2.0, -2.0, 1.0, 1.0), 8.0), Some(RectF64::of(-4.0, -4.0, 3.0, 3.0)));
}

#[test]
fn small_size() {
    let r = RectF64::of(10.0, 10.0, 20.0, 20.0);
    assert_eq!(try_saturating_resize(&r, 0.0), None);
    assert_eq!(try_saturating_resize(&r, 1.0), None);
    assert_eq!(try_saturating_resize(&r, 2.0), None);
    assert_eq!(try_saturating_resize(&r, MAX + 1.0), None);
    assert_eq!(try_saturating_resize(&r, MAX + 2.0), None);
    assert_eq!(try_saturating_resize(&r, MAX + 3.0), None);
}

#[test]
fn same_size() {
    assert_eq!(try_saturating_resize(&RectF64::of(MIN, MIN, MIN + 2.0, MIN + 2.0), 3.0), Some(RectF64::of(MIN, MIN, MIN + 2.0, MIN + 2.0)));
    assert_eq!(try_saturating_resize(&RectF64::of(MIN, MIN, MIN + 3.0, MIN + 3.0), 4.0), Some(RectF64::of(MIN, MIN, MIN + 3.0, MIN + 3.0)));
    assert_eq!(try_saturating_resize(&RectF64::of(MAX - 2.0, MAX - 2.0, MAX, MAX), 3.0), Some(RectF64::of(MAX - 2.0, MAX - 2.0, MAX, MAX)));
    assert_eq!(try_saturating_resize(&RectF64::of(MAX - 3.0, MAX - 3.0, MAX, MAX), 4.0), Some(RectF64::of(MAX - 3.0, MAX - 3.0, MAX, MAX)));
}

#[test]
fn bounds() {
    assert_eq!(try_saturating_resize(&RectF64::of(MIN, MIN, MIN + 2.0, MIN + 2.0), 11.0), Some(RectF64::of(MIN, MIN, MIN + 10.0, MIN + 10.0)));
    assert_eq!(try_saturating_resize(&RectF64::of(MAX - 2.0, MAX - 2.0, MAX, MAX), 11.0), Some(RectF64::of(MAX - 10.0, MAX - 10.0, MAX, MAX)));
}

#[test]
fn small_rect_limits() {
    assert_eq!(try_saturating_resize(&RectF64::of(MIN, MIN, MIN + 2.0, MIN + 2.0), MAX), Some(RectF64::of(MIN, MIN, -2.0, -2.0)));
    assert_eq!(try_saturating_resize(&RectF64::of(MAX - 2.0, MAX - 2.0, MAX, MAX), MAX), Some(RectF64::of(1.0, 1.0, MAX, MAX)));
}

#[test]
fn big_rect_limits() {
    assert_eq!(try_saturating_resize(&RectF64::of(MIN, MIN, -2.0, -2.0), MAX), Some(RectF64::of(MIN, MIN, -2.0, -2.0)));
    assert_eq!(try_saturating_resize(&RectF64::of(MIN + 1.0, MIN + 1.0, -1.0, -1.0), MAX), Some(RectF64::of(MIN + 1.0, MIN + 1.0, -1.0, -1.0)));
    assert_eq!(try_saturating_resize(&RectF64::of(MIN + 2.0, MIN + 2.0, 0.0, 0.0), MAX), Some(RectF64::of(MIN + 2.0, MIN + 2.0, 0.0, 0.0)));
    assert_eq!(try_saturating_resize(&RectF64::of(0.0, 0.0, MAX - 1.0, MAX - 1.0), MAX), Some(RectF64::of(0.0, 0.0, MAX - 1.0, MAX - 1.0)));
    assert_eq!(try_saturating_resize(&RectF64::of(1.0, 1.0, MAX, MAX), MAX), Some(RectF64::of(1.0, 1.0, MAX, MAX)));
}

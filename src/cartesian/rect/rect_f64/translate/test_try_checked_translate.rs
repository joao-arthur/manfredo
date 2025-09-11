use super::try_checked_translate;
use crate::cartesian::{
    point::point_f64::{MAX, MIN, PointF64},
    rect::rect_f64::Rect,
};

#[test]
fn test() {
    assert_eq!(try_checked_translate(&Rect::of(0.0, 0.0, 10.0, 10.0), &PointF64::of(10.0, 20.0)), Some(Rect::of(10.0, 20.0, 20.0, 30.0)));
    assert_eq!(try_checked_translate(&Rect::of(10.0, 20.0, 20.0, 30.0), &PointF64::of(-20.0, -15.0)), Some(Rect::of(-10.0, 5.0, 0.0, 15.0)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_translate(&Rect::of(MIN + 2.0, MIN + 5.0, MAX, MAX), &PointF64::of(-2.0, -5.0)), Some(Rect::of(MIN, MIN, MAX - 2.0, MAX - 5.0)));
    assert_eq!(try_checked_translate(&Rect::of(MIN, MIN, MAX - 2.0, MAX - 5.0), &PointF64::of(2.0, 5.0)), Some(Rect::of(MIN + 2.0, MIN + 5.0, MAX, MAX)));
}

#[test]
fn out_of_bounds() {
    let r = Rect::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX - 10.0);
    assert_eq!(try_checked_translate(&r, &PointF64::of(-20.0, 0.0)), None);
    assert_eq!(try_checked_translate(&r, &PointF64::of(0.0, -20.0)), None);
    assert_eq!(try_checked_translate(&r, &PointF64::of(20.0, 0.0)), None);
    assert_eq!(try_checked_translate(&r, &PointF64::of(0.0, 20.0)), None);
}

#[test]
fn edge_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(try_checked_translate(&r, &PointF64::of(-1.0, 0.0)), None);
    assert_eq!(try_checked_translate(&r, &PointF64::of(0.0, -1.0)), None);
    assert_eq!(try_checked_translate(&r, &PointF64::of(1.0, 0.0)), None);
    assert_eq!(try_checked_translate(&r, &PointF64::of(0.0, 1.0)), None);
}

#[test]
fn limits_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(try_checked_translate(&r, &PointF64::of(MIN, 0.0)), None);
    assert_eq!(try_checked_translate(&r, &PointF64::of(0.0, MIN)), None);
    assert_eq!(try_checked_translate(&r, &PointF64::of(MAX, 0.0)), None);
    assert_eq!(try_checked_translate(&r, &PointF64::of(0.0, MAX)), None);
}

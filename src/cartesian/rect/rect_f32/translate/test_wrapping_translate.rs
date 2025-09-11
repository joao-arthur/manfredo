use super::wrapping_translate;
use crate::cartesian::{
    point::point_f32::{MAX, MIN, PointF32},
    rect::rect_f32::Rect,
};

#[test]
fn test() {
    assert_eq!(wrapping_translate(&Rect::of(0.0, 0.0, 12.0, 15.0), &PointF32::of(5.0, 4.0)), Rect::of(5.0, 4.0, 17.0, 19.0));
    assert_eq!(wrapping_translate(&Rect::of(5.0, 4.0, 17.0, 19.0), &PointF32::of(-4.0, -2.0)), Rect::of(1.0, 2.0, 13.0, 17.0));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_translate(&Rect::of(MIN + 2.0, MIN + 5.0, MAX, MAX), &PointF32::of(-2.0, -5.0)), Rect::of(MIN, MIN, MAX - 2.0, MAX - 5.0));
    assert_eq!(wrapping_translate(&Rect::of(MIN, MIN, MAX - 2.0, MAX - 5.0), &PointF32::of(2.0, 5.0)), Rect::of(MIN + 2.0, MIN + 5.0, MAX, MAX));
}

#[test]
fn out_of_bounds() {
    let r = Rect::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX - 10.0);
    assert_eq!(wrapping_translate(&r, &PointF32::of(-20.0, 0.0)), Rect::of(MAX - 9.0, MIN + 10.0, MAX - 30.0, MAX - 10.0));
    assert_eq!(wrapping_translate(&r, &PointF32::of(0.0, -20.0)), Rect::of(MIN + 10.0, MAX - 9.0, MAX - 10.0, MAX - 30.0));
    assert_eq!(wrapping_translate(&r, &PointF32::of(20.0, 0.0)), Rect::of(MIN + 30.0, MIN + 10.0, MIN + 9.0, MAX - 10.0));
    assert_eq!(wrapping_translate(&r, &PointF32::of(0.0, 20.0)), Rect::of(MIN + 10.0, MIN + 30.0, MAX - 10.0, MIN + 9.0));

    let r_min = Rect::of(MIN, MIN, MIN + 10.0, MIN + 10.0);
    assert_eq!(wrapping_translate(&r_min, &PointF32::of(-20.0, -20.0)), Rect::of(MAX - 19.0, MAX - 19.0, MAX - 9.0, MAX - 9.0));

    let r_max = Rect::of(MAX, MAX, MAX - 10.0, MAX - 10.0);
    assert_eq!(wrapping_translate(&r_max, &PointF32::of(20.0, 20.0)), Rect::of(MIN + 19.0, MIN + 19.0, MIN + 9.0, MIN + 9.0));
}

#[test]
fn edge_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(wrapping_translate(&r, &PointF32::of(-1.0, 0.0)), Rect::of(MAX, MIN, MAX - 1.0, MAX));
    assert_eq!(wrapping_translate(&r, &PointF32::of(0.0, -1.0)), Rect::of(MIN, MAX, MAX, MAX - 1.0));
    assert_eq!(wrapping_translate(&r, &PointF32::of(1.0, 0.0)), Rect::of(MIN + 1.0, MIN, MIN, MAX));
    assert_eq!(wrapping_translate(&r, &PointF32::of(0.0, 1.0)), Rect::of(MIN, MIN + 1.0, MAX, MIN));
}

#[test]
fn limits_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(wrapping_translate(&r, &PointF32::of(MIN, 0.0)), Rect::of(0.0, MIN, -1.0, MAX));
    assert_eq!(wrapping_translate(&r, &PointF32::of(0.0, MIN)), Rect::of(MIN, 0.0, MAX, -1.0));
    assert_eq!(wrapping_translate(&r, &PointF32::of(MAX, 0.0)), Rect::of(-1.0, MIN, -2.0, MAX));
    assert_eq!(wrapping_translate(&r, &PointF32::of(0.0, MAX)), Rect::of(MIN, -1.0, MAX, -2.0));
}

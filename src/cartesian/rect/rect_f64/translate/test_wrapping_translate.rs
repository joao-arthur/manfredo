use super::wrapping_translate;
use crate::cartesian::{
    point::point_f64::{MAX, MIN, PointF64},
    rect::rect_f64::RectF64,
};

#[test]
fn test() {
    assert_eq!(wrapping_translate(&RectF64::of(0.0, 0.0, 12.0, 15.0), &PointF64::of(5.0, 4.0)), RectF64::of(5.0, 4.0, 17.0, 19.0));
    assert_eq!(wrapping_translate(&RectF64::of(5.0, 4.0, 17.0, 19.0), &PointF64::of(-4.0, -2.0)), RectF64::of(1.0, 2.0, 13.0, 17.0));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_translate(&RectF64::of(MIN + 2.0, MIN + 5.0, MAX, MAX), &PointF64::of(-2.0, -5.0)), RectF64::of(MIN, MIN, MAX - 2.0, MAX - 5.0));
    assert_eq!(wrapping_translate(&RectF64::of(MIN, MIN, MAX - 2.0, MAX - 5.0), &PointF64::of(2.0, 5.0)), RectF64::of(MIN + 2.0, MIN + 5.0, MAX, MAX));
}

#[test]
fn out_of_bounds() {
    let r = RectF64::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX - 10.0);
    assert_eq!(wrapping_translate(&r, &PointF64::of(-20.0, 0.0)), RectF64::of(MAX - 9.0, MIN + 10.0, MAX - 30.0, MAX - 10.0));
    assert_eq!(wrapping_translate(&r, &PointF64::of(0.0, -20.0)), RectF64::of(MIN + 10.0, MAX - 9.0, MAX - 10.0, MAX - 30.0));
    assert_eq!(wrapping_translate(&r, &PointF64::of(20.0, 0.0)), RectF64::of(MIN + 30.0, MIN + 10.0, MIN + 9.0, MAX - 10.0));
    assert_eq!(wrapping_translate(&r, &PointF64::of(0.0, 20.0)), RectF64::of(MIN + 10.0, MIN + 30.0, MAX - 10.0, MIN + 9.0));

    let r_min = RectF64::of(MIN, MIN, MIN + 10.0, MIN + 10.0);
    assert_eq!(wrapping_translate(&r_min, &PointF64::of(-20.0, -20.0)), RectF64::of(MAX - 19.0, MAX - 19.0, MAX - 9.0, MAX - 9.0));

    let r_max = RectF64::of(MAX, MAX, MAX - 10.0, MAX - 10.0);
    assert_eq!(wrapping_translate(&r_max, &PointF64::of(20.0, 20.0)), RectF64::of(MIN + 19.0, MIN + 19.0, MIN + 9.0, MIN + 9.0));
}

#[test]
fn limits_out_of_bounds() {
    let r = RectF64::largest();
    assert_eq!(wrapping_translate(&r, &PointF64::of(MIN, 0.0)), RectF64::of(0.0, MIN, -1.0, MAX));
    assert_eq!(wrapping_translate(&r, &PointF64::of(0.0, MIN)), RectF64::of(MIN, 0.0, MAX, -1.0));
    assert_eq!(wrapping_translate(&r, &PointF64::of(MAX, 0.0)), RectF64::of(-1.0, MIN, -2.0, MAX));
    assert_eq!(wrapping_translate(&r, &PointF64::of(0.0, MAX)), RectF64::of(MIN, -1.0, MAX, -2.0));
}

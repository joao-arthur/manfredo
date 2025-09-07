use super::saturating_translate;
use crate::cartesian::{
    point::point_f32::{MAX, MIN, PointF32},
    rect::rect_f32::RectF32,
};

#[test]
fn test() {
    assert_eq!(saturating_translate(&RectF32::of(0.0, 0.0, 10.0, 10.0), &PointF32::of(10.0, 20.0)), RectF32::of(10.0, 20.0, 20.0, 30.0));
    assert_eq!(saturating_translate(&RectF32::of(10.0, 20.0, 20.0, 30.0), &PointF32::of(-20.0, -15.0)), RectF32::of(-10.0, 5.0, 0.0, 15.0));
}

#[test]
fn to_bounds() {
    assert_eq!(saturating_translate(&RectF32::of(MIN + 2.0, MIN + 5.0, 0.0, 0.0), &PointF32::of(-2.0, -5.0)), RectF32::of(MIN, MIN, -2.0, -5.0));
    assert_eq!(saturating_translate(&RectF32::of(MIN, MIN, -2.0, -5.0), &PointF32::of(2.0, 5.0)), RectF32::of(MIN + 2.0, MIN + 5.0, 0.0, 0.0));
    assert_eq!(saturating_translate(&RectF32::of(2.0, 5.0, MAX, MAX), &PointF32::of(-2.0, -5.0)), RectF32::of(0.0, 0.0, MAX - 2.0, MAX - 5.0));
    assert_eq!(saturating_translate(&RectF32::of(0.0, 0.0, MAX - 2.0, MAX - 5.0), &PointF32::of(2.0, 5.0)), RectF32::of(2.0, 5.0, MAX, MAX));
}

#[test]
fn out_of_bounds() {
    let r_min = RectF32::of(MIN + 10.0, MIN + 10.0, 0.0, 0.0);
    assert_eq!(saturating_translate(&r_min, &PointF32::of(-20.0, 0.0)), RectF32::of(MIN, MIN + 10.0, -10.0, 0.0));
    assert_eq!(saturating_translate(&r_min, &PointF32::of(0.0, -20.0)), RectF32::of(MIN + 10.0, MIN, 0.0, -10.0));
    assert_eq!(saturating_translate(&r_min, &PointF32::of(20.0, 0.0)), RectF32::of(MIN + 30.0, MIN + 10.0, 20.0, 0.0));
    assert_eq!(saturating_translate(&r_min, &PointF32::of(0.0, 20.0)), RectF32::of(MIN + 10.0, MIN + 30.0, 0.0, 20.0));

    let r_max = RectF32::of(0.0, 0.0, MAX - 10.0, MAX - 10.0);
    assert_eq!(saturating_translate(&r_max, &PointF32::of(-20.0, 0.0)), RectF32::of(-20.0, 0.0, MAX - 30.0, MAX - 10.0));
    assert_eq!(saturating_translate(&r_max, &PointF32::of(0.0, -20.0)), RectF32::of(0.0, -20.0, MAX - 10.0, MAX - 30.0));
    assert_eq!(saturating_translate(&r_max, &PointF32::of(20.0, 0.0)), RectF32::of(10.0, 0.0, MAX, MAX - 10.0));
    assert_eq!(saturating_translate(&r_max, &PointF32::of(0.0, 20.0)), RectF32::of(0.0, 10.0, MAX - 10.0, MAX));
}

#[test]
fn limits_out_of_bounds() {
    let r = RectF32::largest();
    assert_eq!(saturating_translate(&r, &PointF32::of(MIN, 0.0)), RectF32::of(MIN, MIN, -1.0, -1.0));
    assert_eq!(saturating_translate(&r, &PointF32::of(0.0, MIN)), RectF32::of(MIN, MIN, -1.0, -1.0));
    assert_eq!(saturating_translate(&r, &PointF32::of(MAX, 0.0)), RectF32::of(-1.0, MIN, MAX - 1.0, -1.0));
    assert_eq!(saturating_translate(&r, &PointF32::of(0.0, MAX)), RectF32::of(MIN, -1.0, -1.0, MAX - 1.0));
}

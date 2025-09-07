use super::saturating_translate;
use crate::cartesian::{point::point_i64::PointI64, rect::rect_i64::RectI64};

const MIN: i64 = i64::MIN;
const MAX: i64 = i64::MAX;

#[test]
fn test() {
    assert_eq!(saturating_translate(&RectI64::of(5, 9, 13, 37), &PointI64::of(-10, -20)), RectI64::of(-5, -11, 3, 17));
    assert_eq!(saturating_translate(&RectI64::of(-5, -11, 3, 17), &PointI64::of(6, -19)), RectI64::of(1, -30, 9, -2));
}

#[test]
fn to_bounds() {
    assert_eq!(saturating_translate(&RectI64::of(MIN + 2, MIN + 5, MAX, MAX), &PointI64::of(-2, -5)), RectI64::of(MIN, MIN, MAX - 2, MAX - 5));
    assert_eq!(saturating_translate(&RectI64::of(MIN, MIN, MAX - 2, MAX - 5), &PointI64::of(2, 5)), RectI64::of(MIN + 2, MIN + 5, MAX, MAX));
}

#[test]
fn out_of_bounds() {
    let r = RectI64::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10);
    assert_eq!(saturating_translate(&r, &PointI64::of(-20, 0)), RectI64::of(MIN, MIN + 10, MAX - 20, MAX - 10));
    assert_eq!(saturating_translate(&r, &PointI64::of(0, -20)), RectI64::of(MIN + 10, MIN, MAX - 10, MAX - 20));
    assert_eq!(saturating_translate(&r, &PointI64::of(20, 0)), RectI64::of(MIN + 20, MIN + 10, MAX, MAX - 10));
    assert_eq!(saturating_translate(&r, &PointI64::of(0, 20)), RectI64::of(MIN + 10, MIN + 20, MAX - 10, MAX));
}

#[test]
fn limits_out_of_bounds() {
    let r = RectI64::largest();
    assert_eq!(saturating_translate(&r, &PointI64::of(MIN, 0)), RectI64::largest());
    assert_eq!(saturating_translate(&r, &PointI64::of(0, MIN)), RectI64::largest());
    assert_eq!(saturating_translate(&r, &PointI64::of(MAX, 0)), RectI64::largest());
    assert_eq!(saturating_translate(&r, &PointI64::of(0, MAX)), RectI64::largest());
}

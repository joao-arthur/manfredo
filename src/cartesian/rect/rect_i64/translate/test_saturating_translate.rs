use super::saturating_translate;
use crate::cartesian::{point::point_i64::PointI64, rect::rect_i64::RectI64};

#[test]
fn test_saturating_translate() {
    assert_eq!(saturating_translate(&RectI64::of(5, 9, 13, 37), &PointI64::of(-10, -20)), RectI64::of(-5, -11, 3, 17));
    assert_eq!(saturating_translate(&RectI64::of(-5, -11, 3, 17), &PointI64::of(6, -19)), RectI64::of(1, -30, 9, -2));
}

#[test]
fn saturating_translate_to_bounds() {
    assert_eq!(saturating_translate(&RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX, i64::MAX), &PointI64::of(-2, -5)), RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 5));
    assert_eq!(saturating_translate(&RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 5), &PointI64::of(2, 5)), RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX, i64::MAX));
}

#[test]
fn saturating_translate_out_of_bounds() {
    let r = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 10, i64::MAX - 10);
    assert_eq!(saturating_translate(&r, &PointI64::of(-20, 0)), RectI64::of(i64::MIN, i64::MIN + 10, i64::MAX - 20, i64::MAX - 10));
    assert_eq!(saturating_translate(&r, &PointI64::of(0, -20)), RectI64::of(i64::MIN + 10, i64::MIN, i64::MAX - 10, i64::MAX - 20));
    assert_eq!(saturating_translate(&r, &PointI64::of(20, 0)), RectI64::of(i64::MIN + 20, i64::MIN + 10, i64::MAX, i64::MAX - 10));
    assert_eq!(saturating_translate(&r, &PointI64::of(0, 20)), RectI64::of(i64::MIN + 10, i64::MIN + 20, i64::MAX - 10, i64::MAX));
}

#[test]
fn saturating_translate_limits_out_of_bounds() {
    let r = RectI64::largest();
    assert_eq!(saturating_translate(&r, &PointI64::of(i64::MIN, 0)), RectI64::largest());
    assert_eq!(saturating_translate(&r, &PointI64::of(0, i64::MIN)), RectI64::largest());
    assert_eq!(saturating_translate(&r, &PointI64::of(i64::MAX, 0)), RectI64::largest());
    assert_eq!(saturating_translate(&r, &PointI64::of(0, i64::MAX)), RectI64::largest());
}

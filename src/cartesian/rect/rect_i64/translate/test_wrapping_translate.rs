use super::wrapping_translate;
use crate::cartesian::{point::point_i64::PointI64, rect::rect_i64::RectI64};

#[test]
fn test_wrapping_translate() {
    assert_eq!(wrapping_translate(&RectI64::of(0, 0, 12, 15), &PointI64::of(5, 4)), RectI64::of(5, 4, 17, 19));
    assert_eq!(wrapping_translate(&RectI64::of(5, 4, 17, 19), &PointI64::of(-4, -2)), RectI64::of(1, 2, 13, 17));
}

#[test]
fn wrapping_translate_to_bounds() {
    assert_eq!(wrapping_translate(&RectI64::of(2, 5, i64::MAX, i64::MAX), &PointI64::of(-2, -5)), RectI64::of(0, 0, i64::MAX - 2, i64::MAX - 5));
    assert_eq!(wrapping_translate(&RectI64::of(0, 0, i64::MAX - 2, i64::MAX - 5), &PointI64::of(2, 5)), RectI64::of(2, 5, i64::MAX, i64::MAX));
}

#[test]
fn wrapping_translate_out_of_bounds() {
    let r = RectI64::of(10, 10, i64::MAX - 10, i64::MAX - 10);
    assert_eq!(wrapping_translate(&r, &PointI64::of(-20, 0)), RectI64::of(i64::MAX - 9, 10, i64::MAX - 30, i64::MAX - 10));
    assert_eq!(wrapping_translate(&r, &PointI64::of(0, -20)), RectI64::of(10, i64::MAX - 9, i64::MAX - 10, i64::MAX - 30));
    assert_eq!(wrapping_translate(&r, &PointI64::of(20, 0)), RectI64::of(30, 10, 9, i64::MAX - 10));
    assert_eq!(wrapping_translate(&r, &PointI64::of(0, 20)), RectI64::of(10, 30, i64::MAX - 10, 9));
}

#[test]
fn wrapping_translate_limits_out_of_bounds() {
    let r = RectI64::largest();
    assert_eq!(wrapping_translate(&r, &PointI64::of(i64::MIN, 0)), RectI64::of(i64::MAX / 2 + 1, 0, i64::MAX / 2, i64::MAX));
    assert_eq!(wrapping_translate(&r, &PointI64::of(0, i64::MIN)), RectI64::of(0, i64::MAX / 2 + 1, i64::MAX, i64::MAX / 2));
    assert_eq!(wrapping_translate(&r, &PointI64::of(i64::MAX, 0)), RectI64::of(i64::MAX / 2, 0, i64::MAX / 2 - 1, i64::MAX));
    assert_eq!(wrapping_translate(&r, &PointI64::of(0, i64::MAX)), RectI64::of(0, i64::MAX / 2, i64::MAX, i64::MAX / 2 - 1));
}

use super::saturating_translate;
use crate::matrix::{point::point_i32::PointI32, rect::rect_u32::RectU32};

const MAX: u32 = u32::MAX;

#[test]
fn test() {
    assert_eq!(saturating_translate(&RectU32::of(0, 0, 12, 15), &PointI32::of(5, 4)), RectU32::of(5, 4, 17, 19));
    assert_eq!(saturating_translate(&RectU32::of(5, 4, 17, 19), &PointI32::of(-4, -2)), RectU32::of(1, 2, 13, 17));
}

#[test]
fn to_bounds() {
    assert_eq!(saturating_translate(&RectU32::of(2, 5, MAX, MAX), &PointI32::of(-2, -5)), RectU32::of(0, 0, MAX - 2, MAX - 5));
    assert_eq!(saturating_translate(&RectU32::of(0, 0, MAX - 2, MAX - 5), &PointI32::of(2, 5)), RectU32::of(2, 5, MAX, MAX));
}

#[test]
fn out_of_bounds() {
    let r = RectU32::of(10, 10, MAX - 10, MAX - 10);
    assert_eq!(saturating_translate(&r, &PointI32::of(-20, 0)), RectU32::of(0, 10, MAX - 20, MAX - 10));
    assert_eq!(saturating_translate(&r, &PointI32::of(0, -20)), RectU32::of(10, 0, MAX - 10, MAX - 20));
    assert_eq!(saturating_translate(&r, &PointI32::of(20, 0)), RectU32::of(20, 10, MAX, MAX - 10));
    assert_eq!(saturating_translate(&r, &PointI32::of(0, 20)), RectU32::of(10, 20, MAX - 10, MAX));
}

#[test]
fn limits_out_of_bounds() {
    let r = RectU32::largest();
    assert_eq!(saturating_translate(&r, &PointI32::of(i32::MIN, 0)), RectU32::largest());
    assert_eq!(saturating_translate(&r, &PointI32::of(0, i32::MIN)), RectU32::largest());
    assert_eq!(saturating_translate(&r, &PointI32::of(i32::MAX, 0)), RectU32::largest());
    assert_eq!(saturating_translate(&r, &PointI32::of(0, i32::MAX)), RectU32::largest());
}

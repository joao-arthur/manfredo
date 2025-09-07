use super::wrapping_translate;
use crate::matrix::{point::point_i32::PointI32, rect::rect_i32::RectI32};

const MIN: i32 = i32::MIN;
const MAX: i32 = i32::MAX;

#[test]
fn test() {
    assert_eq!(wrapping_translate(&RectI32::of(5, 9, 13, 37), &PointI32::of(-10, -20)), RectI32::of(-5, -11, 3, 17));
    assert_eq!(wrapping_translate(&RectI32::of(-5, -11, 3, 17), &PointI32::of(6, -19)), RectI32::of(1, -30, 9, -2));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_translate(&RectI32::of(MIN + 2, MIN + 5, MAX, MAX), &PointI32::of(-2, -5)), RectI32::of(MIN, MIN, MAX - 2, MAX - 5));
    assert_eq!(wrapping_translate(&RectI32::of(MIN, MIN, MAX - 2, MAX - 5), &PointI32::of(2, 5)), RectI32::of(MIN + 2, MIN + 5, MAX, MAX));
}

#[test]
fn out_of_bounds() {
    let r = RectI32::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10);
    assert_eq!(wrapping_translate(&r, &PointI32::of(-20, 0)), RectI32::of(MAX - 9, MIN + 10, MAX - 30, MAX - 10));
    assert_eq!(wrapping_translate(&r, &PointI32::of(0, -20)), RectI32::of(MIN + 10, MAX - 9, MAX - 10, MAX - 30));
    assert_eq!(wrapping_translate(&r, &PointI32::of(20, 0)), RectI32::of(MIN + 30, MIN + 10, MIN + 9, MAX - 10));
    assert_eq!(wrapping_translate(&r, &PointI32::of(0, 20)), RectI32::of(MIN + 10, MIN + 30, MAX - 10, MIN + 9));
}

#[test]
fn limits_out_of_bounds() {
    let r = RectI32::largest();
    assert_eq!(wrapping_translate(&r, &PointI32::of(MIN, 0)), RectI32::of(0, MIN, -1, MAX));
    assert_eq!(wrapping_translate(&r, &PointI32::of(0, MIN)), RectI32::of(MIN, 0, MAX, -1));
    assert_eq!(wrapping_translate(&r, &PointI32::of(MAX, 0)), RectI32::of(-1, MIN, -2, MAX));
    assert_eq!(wrapping_translate(&r, &PointI32::of(0, MAX)), RectI32::of(MIN, -1, MAX, -2));
}

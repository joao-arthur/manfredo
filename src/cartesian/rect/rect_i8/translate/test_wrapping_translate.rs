use super::wrapping_translate;
use crate::cartesian::{point::point_i8::PointI8, rect::rect_i8::RectI8};

#[test]
fn test() {
    assert_eq!(wrapping_translate(&RectI8::of(5, 9, 13, 37), &PointI8::of(-10, -20)), RectI8::of(-5, -11, 3, 17));
    assert_eq!(wrapping_translate(&RectI8::of(-5, -11, 3, 17), &PointI8::of(6, -19)), RectI8::of(1, -30, 9, -2));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_translate(&RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX, i8::MAX), &PointI8::of(-2, -5)), RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 5));
    assert_eq!(wrapping_translate(&RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 5), &PointI8::of(2, 5)), RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX, i8::MAX));
}

#[test]
fn out_of_bounds() {
    let r = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 10, i8::MAX - 10);
    assert_eq!(wrapping_translate(&r, &PointI8::of(-20, 0)), RectI8::of(i8::MAX - 9, i8::MIN + 10, i8::MAX - 30, i8::MAX - 10));
    assert_eq!(wrapping_translate(&r, &PointI8::of(0, -20)), RectI8::of(i8::MIN + 10, i8::MAX - 9, i8::MAX - 10, i8::MAX - 30));
    assert_eq!(wrapping_translate(&r, &PointI8::of(20, 0)), RectI8::of(i8::MIN + 30, i8::MIN + 10, i8::MIN + 9, i8::MAX - 10));
    assert_eq!(wrapping_translate(&r, &PointI8::of(0, 20)), RectI8::of(i8::MIN + 10, i8::MIN + 30, i8::MAX - 10, i8::MIN + 9));
}

#[test]
fn limits_out_of_bounds() {
    let r = RectI8::largest();
    assert_eq!(wrapping_translate(&r, &PointI8::of(i8::MIN, 0)), RectI8::of(0, i8::MIN, -1, i8::MAX));
    assert_eq!(wrapping_translate(&r, &PointI8::of(0, i8::MIN)), RectI8::of(i8::MIN, 0, i8::MAX, -1));
    assert_eq!(wrapping_translate(&r, &PointI8::of(i8::MAX, 0)), RectI8::of(-1, i8::MIN, -2, i8::MAX));
    assert_eq!(wrapping_translate(&r, &PointI8::of(0, i8::MAX)), RectI8::of(i8::MIN, -1, i8::MAX, -2));
}

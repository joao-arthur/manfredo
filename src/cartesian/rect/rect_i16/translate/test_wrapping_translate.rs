use super::wrapping_translate;
use crate::cartesian::{point::point_i16::PointI16, rect::rect_i16::RectI16};

#[test]
fn test() {
    assert_eq!(wrapping_translate(&RectI16::of(5, 9, 13, 37), &PointI16::of(-10, -20)), RectI16::of(-5, -11, 3, 17));
    assert_eq!(wrapping_translate(&RectI16::of(-5, -11, 3, 17), &PointI16::of(6, -19)), RectI16::of(1, -30, 9, -2));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_translate(&RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX), &PointI16::of(-2, -5)), RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5));
    assert_eq!(wrapping_translate(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5), &PointI16::of(2, 5)), RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX));
}

#[test]
fn out_of_bounds() {
    let r = RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 10, i16::MAX - 10);
    assert_eq!(wrapping_translate(&r, &PointI16::of(-20, 0)), RectI16::of(i16::MAX - 9, i16::MIN + 10, i16::MAX - 30, i16::MAX - 10));
    assert_eq!(wrapping_translate(&r, &PointI16::of(0, -20)), RectI16::of(i16::MIN + 10, i16::MAX - 9, i16::MAX - 10, i16::MAX - 30));
    assert_eq!(wrapping_translate(&r, &PointI16::of(20, 0)), RectI16::of(i16::MIN + 30, i16::MIN + 10, i16::MIN + 9, i16::MAX - 10));
    assert_eq!(wrapping_translate(&r, &PointI16::of(0, 20)), RectI16::of(i16::MIN + 10, i16::MIN + 30, i16::MAX - 10, i16::MIN + 9));
}

#[test]
fn limits_out_of_bounds() {
    let r = RectI16::largest();
    assert_eq!(wrapping_translate(&r, &PointI16::of(i16::MIN, 0)), RectI16::of(0, i16::MIN, -1, i16::MAX));
    assert_eq!(wrapping_translate(&r, &PointI16::of(0, i16::MIN)), RectI16::of(i16::MIN, 0, i16::MAX, -1));
    assert_eq!(wrapping_translate(&r, &PointI16::of(i16::MAX, 0)), RectI16::of(-1, i16::MIN, -2, i16::MAX));
    assert_eq!(wrapping_translate(&r, &PointI16::of(0, i16::MAX)), RectI16::of(i16::MIN, -1, i16::MAX, -2));
}

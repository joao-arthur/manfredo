use super::wrapping_translate;
use crate::matrix::{point::point_i16::PointI16, rect::rect_u16::RectU16};

#[test]
fn test() {
    assert_eq!(wrapping_translate(&RectU16::of(0, 0, 12, 15), &PointI16::of(5, 4)), RectU16::of(5, 4, 17, 19));
    assert_eq!(wrapping_translate(&RectU16::of(5, 4, 17, 19), &PointI16::of(-4, -2)), RectU16::of(1, 2, 13, 17));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_translate(&RectU16::of(2, 5, u16::MAX, u16::MAX), &PointI16::of(-2, -5)), RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5));
    assert_eq!(wrapping_translate(&RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5), &PointI16::of(2, 5)), RectU16::of(2, 5, u16::MAX, u16::MAX));
}

#[test]
fn out_of_bounds() {
    let r = RectU16::of(10, 10, u16::MAX - 10, u16::MAX - 10);
    assert_eq!(wrapping_translate(&r, &PointI16::of(-20, 0)), RectU16::of(u16::MAX - 9, 10, u16::MAX - 30, u16::MAX - 10));
    assert_eq!(wrapping_translate(&r, &PointI16::of(0, -20)), RectU16::of(10, u16::MAX - 9, u16::MAX - 10, u16::MAX - 30));
    assert_eq!(wrapping_translate(&r, &PointI16::of(20, 0)), RectU16::of(30, 10, 9, u16::MAX - 10));
    assert_eq!(wrapping_translate(&r, &PointI16::of(0, 20)), RectU16::of(10, 30, u16::MAX - 10, 9));
}

#[test]
fn limits_out_of_bounds() {
    let r = RectU16::largest();
    assert_eq!(wrapping_translate(&r, &PointI16::of(i16::MIN, 0)), RectU16::of(u16::MAX / 2 + 1, 0, u16::MAX / 2, u16::MAX));
    assert_eq!(wrapping_translate(&r, &PointI16::of(0, i16::MIN)), RectU16::of(0, u16::MAX / 2 + 1, u16::MAX, u16::MAX / 2));
    assert_eq!(wrapping_translate(&r, &PointI16::of(i16::MAX, 0)), RectU16::of(u16::MAX / 2, 0, u16::MAX / 2 - 1, u16::MAX));
    assert_eq!(wrapping_translate(&r, &PointI16::of(0, i16::MAX)), RectU16::of(0, u16::MAX / 2, u16::MAX, u16::MAX / 2 - 1));
}

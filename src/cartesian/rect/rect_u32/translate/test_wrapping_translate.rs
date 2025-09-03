use super::wrapping_translate;
use crate::cartesian::{point::point_i32::PointI32, rect::rect_u32::RectU32};

#[test]
fn test_wrapping_translate() {
    assert_eq!(wrapping_translate(&RectU32::of(0, 0, 12, 15), &PointI32::of(5, 4)), RectU32::of(5, 4, 17, 19));
    assert_eq!(wrapping_translate(&RectU32::of(5, 4, 17, 19), &PointI32::of(-4, -2)), RectU32::of(1, 2, 13, 17));
}

#[test]
fn wrapping_translate_to_bounds() {
    assert_eq!(wrapping_translate(&RectU32::of(2, 5, u32::MAX, u32::MAX), &PointI32::of(-2, -5)), RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5));
    assert_eq!(wrapping_translate(&RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5), &PointI32::of(2, 5)), RectU32::of(2, 5, u32::MAX, u32::MAX));
}

#[test]
fn wrapping_translate_out_of_bounds() {
    let r = RectU32::of(10, 10, u32::MAX - 10, u32::MAX - 10);
    assert_eq!(wrapping_translate(&r, &PointI32::of(-20, 0)), RectU32::of(u32::MAX - 9, 10, u32::MAX - 30, u32::MAX - 10));
    assert_eq!(wrapping_translate(&r, &PointI32::of(0, -20)), RectU32::of(10, u32::MAX - 9, u32::MAX - 10, u32::MAX - 30));
    assert_eq!(wrapping_translate(&r, &PointI32::of(20, 0)), RectU32::of(30, 10, 9, u32::MAX - 10));
    assert_eq!(wrapping_translate(&r, &PointI32::of(0, 20)), RectU32::of(10, 30, u32::MAX - 10, 9));
}

#[test]
fn wrapping_translate_limits_out_of_bounds() {
    let r = RectU32::largest();
    assert_eq!(wrapping_translate(&r, &PointI32::of(i32::MIN, 0)), RectU32::of(u32::MAX / 2 + 1, 0, u32::MAX / 2, u32::MAX));
    assert_eq!(wrapping_translate(&r, &PointI32::of(0, i32::MIN)), RectU32::of(0, u32::MAX / 2 + 1, u32::MAX, u32::MAX / 2));
    assert_eq!(wrapping_translate(&r, &PointI32::of(i32::MAX, 0)), RectU32::of(u32::MAX / 2, 0, u32::MAX / 2 - 1, u32::MAX));
    assert_eq!(wrapping_translate(&r, &PointI32::of(0, i32::MAX)), RectU32::of(0, u32::MAX / 2, u32::MAX, u32::MAX / 2 - 1));
}

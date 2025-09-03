use super::saturating_translate;
use crate::cartesian::{point::point_i8::PointI8, rect::rect_u8::RectU8};

#[test]
fn test_saturating_translate() {
    assert_eq!(saturating_translate(&RectU8::of(0, 0, 12, 15), &PointI8::of(5, 4)), RectU8::of(5, 4, 17, 19));
    assert_eq!(saturating_translate(&RectU8::of(5, 4, 17, 19), &PointI8::of(-4, -2)), RectU8::of(1, 2, 13, 17));
}

#[test]
fn saturating_translate_to_bounds() {
    assert_eq!(saturating_translate(&RectU8::of(2, 5, u8::MAX, u8::MAX), &PointI8::of(-2, -5)), RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 5));
    assert_eq!(saturating_translate(&RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 5), &PointI8::of(2, 5)), RectU8::of(2, 5, u8::MAX, u8::MAX));
}

#[test]
fn saturating_translate_out_of_bounds() {
    let r = RectU8::of(10, 10, u8::MAX - 10, u8::MAX - 10);
    assert_eq!(saturating_translate(&r, &PointI8::of(-20, 0)), RectU8::of(0, 10, u8::MAX - 20, u8::MAX - 10));
    assert_eq!(saturating_translate(&r, &PointI8::of(0, -20)), RectU8::of(10, 0, u8::MAX - 10, u8::MAX - 20));
    assert_eq!(saturating_translate(&r, &PointI8::of(20, 0)), RectU8::of(20, 10, u8::MAX, u8::MAX - 10));
    assert_eq!(saturating_translate(&r, &PointI8::of(0, 20)), RectU8::of(10, 20, u8::MAX - 10, u8::MAX));
}

#[test]
fn saturating_translate_limits_out_of_bounds() {
    let r = RectU8::largest();
    assert_eq!(saturating_translate(&r, &PointI8::of(i8::MIN, 0)), RectU8::largest());
    assert_eq!(saturating_translate(&r, &PointI8::of(0, i8::MIN)), RectU8::largest());
    assert_eq!(saturating_translate(&r, &PointI8::of(i8::MAX, 0)), RectU8::largest());
    assert_eq!(saturating_translate(&r, &PointI8::of(0, i8::MAX)), RectU8::largest());
}

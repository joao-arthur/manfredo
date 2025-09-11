use super::saturating_translate;
use crate::matrix::{point::point_i8::PointI8, rect::rect_u8::Rect};

const MAX: u8 = u8::MAX;

#[test]
fn test() {
    assert_eq!(saturating_translate(&Rect::of(0, 0, 12, 15), &PointI8::of(5, 4)), Rect::of(5, 4, 17, 19));
    assert_eq!(saturating_translate(&Rect::of(5, 4, 17, 19), &PointI8::of(-4, -2)), Rect::of(1, 2, 13, 17));
}

#[test]
fn to_bounds() {
    assert_eq!(saturating_translate(&Rect::of(2, 5, MAX, MAX), &PointI8::of(-2, -5)), Rect::of(0, 0, MAX - 2, MAX - 5));
    assert_eq!(saturating_translate(&Rect::of(0, 0, MAX - 2, MAX - 5), &PointI8::of(2, 5)), Rect::of(2, 5, MAX, MAX));
}

#[test]
fn out_of_bounds() {
    let r = Rect::of(10, 10, MAX - 10, MAX - 10);
    assert_eq!(saturating_translate(&r, &PointI8::of(-20, 0)), Rect::of(0, 10, MAX - 20, MAX - 10));
    assert_eq!(saturating_translate(&r, &PointI8::of(0, -20)), Rect::of(10, 0, MAX - 10, MAX - 20));
    assert_eq!(saturating_translate(&r, &PointI8::of(20, 0)), Rect::of(20, 10, MAX, MAX - 10));
    assert_eq!(saturating_translate(&r, &PointI8::of(0, 20)), Rect::of(10, 20, MAX - 10, MAX));
}

#[test]
fn limits_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(saturating_translate(&r, &PointI8::of(i8::MIN, 0)), Rect::largest());
    assert_eq!(saturating_translate(&r, &PointI8::of(0, i8::MIN)), Rect::largest());
    assert_eq!(saturating_translate(&r, &PointI8::of(i8::MAX, 0)), Rect::largest());
    assert_eq!(saturating_translate(&r, &PointI8::of(0, i8::MAX)), Rect::largest());
}

use super::try_checked_translate;
use crate::matrix::{point::point_i8::PointI8, rect::rect_i8::RectI8};

const MIN: i8 = i8::MIN;
const MAX: i8 = i8::MAX;

#[test]
fn test() {
    assert_eq!(try_checked_translate(&RectI8::of(5, 9, 13, 37), &PointI8::of(-10, -20)), Some(RectI8::of(-5, -11, 3, 17)));
    assert_eq!(try_checked_translate(&RectI8::of(-5, -11, 3, 17), &PointI8::of(6, -19)), Some(RectI8::of(1, -30, 9, -2)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_translate(&RectI8::of(MIN + 2, MIN + 5, MAX, MAX), &PointI8::of(-2, -5)), Some(RectI8::of(MIN, MIN, MAX - 2, MAX - 5)));
    assert_eq!(try_checked_translate(&RectI8::of(MIN, MIN, MAX - 2, MAX - 5), &PointI8::of(2, 5)), Some(RectI8::of(MIN + 2, MIN + 5, MAX, MAX)));
}

#[test]
fn out_of_bounds() {
    let r = RectI8::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10);
    assert_eq!(try_checked_translate(&r, &PointI8::of(-20, 0)), None);
    assert_eq!(try_checked_translate(&r, &PointI8::of(0, -20)), None);
    assert_eq!(try_checked_translate(&r, &PointI8::of(20, 0)), None);
    assert_eq!(try_checked_translate(&r, &PointI8::of(0, 20)), None);
}

#[test]
fn limits_out_of_bounds() {
    let r = RectI8::largest();
    assert_eq!(try_checked_translate(&r, &PointI8::of(MIN, 0)), None);
    assert_eq!(try_checked_translate(&r, &PointI8::of(0, MIN)), None);
    assert_eq!(try_checked_translate(&r, &PointI8::of(MAX, 0)), None);
    assert_eq!(try_checked_translate(&r, &PointI8::of(0, MAX)), None);
}

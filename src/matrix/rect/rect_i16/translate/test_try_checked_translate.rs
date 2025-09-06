use super::try_checked_translate;
use crate::matrix::{point::point_i16::PointI16, rect::rect_i16::RectI16};

#[test]
fn test() {
    assert_eq!(try_checked_translate(&RectI16::of(5, 9, 13, 37), &PointI16::of(-10, -20)), Some(RectI16::of(-5, -11, 3, 17)));
    assert_eq!(try_checked_translate(&RectI16::of(-5, -11, 3, 17), &PointI16::of(6, -19)), Some(RectI16::of(1, -30, 9, -2)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_translate(&RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX), &PointI16::of(-2, -5)), Some(RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5)));
    assert_eq!(try_checked_translate(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5), &PointI16::of(2, 5)), Some(RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX)));
}

#[test]
fn out_of_bounds() {
    let r = RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 10, i16::MAX - 10);
    assert_eq!(try_checked_translate(&r, &PointI16::of(-20, 0)), None);
    assert_eq!(try_checked_translate(&r, &PointI16::of(0, -20)), None);
    assert_eq!(try_checked_translate(&r, &PointI16::of(20, 0)), None);
    assert_eq!(try_checked_translate(&r, &PointI16::of(0, 20)), None);
}

#[test]
fn limits_out_of_bounds() {
    let r = RectI16::largest();
    assert_eq!(try_checked_translate(&r, &PointI16::of(i16::MIN, 0)), None);
    assert_eq!(try_checked_translate(&r, &PointI16::of(0, i16::MIN)), None);
    assert_eq!(try_checked_translate(&r, &PointI16::of(i16::MAX, 0)), None);
    assert_eq!(try_checked_translate(&r, &PointI16::of(0, i16::MAX)), None);
}

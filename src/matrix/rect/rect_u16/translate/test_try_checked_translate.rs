use super::try_checked_translate;
use crate::matrix::{point::point_i16::PointI16, rect::rect_u16::RectU16};

#[test]
fn test() {
    assert_eq!(try_checked_translate(&RectU16::of(0, 0, 12, 15), &PointI16::of(5, 4)), Some(RectU16::of(5, 4, 17, 19)));
    assert_eq!(try_checked_translate(&RectU16::of(5, 4, 17, 19), &PointI16::of(-4, -2)), Some(RectU16::of(1, 2, 13, 17)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_translate(&RectU16::of(2, 5, u16::MAX, u16::MAX), &PointI16::of(-2, -5)), Some(RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5)));
    assert_eq!(try_checked_translate(&RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5), &PointI16::of(2, 5)), Some(RectU16::of(2, 5, u16::MAX, u16::MAX)));
}

#[test]
fn out_of_bounds() {
    let r = RectU16::of(10, 10, u16::MAX - 10, u16::MAX - 10);
    assert_eq!(try_checked_translate(&r, &PointI16::of(-20, 0)), None);
    assert_eq!(try_checked_translate(&r, &PointI16::of(0, -20)), None);
    assert_eq!(try_checked_translate(&r, &PointI16::of(20, 0)), None);
    assert_eq!(try_checked_translate(&r, &PointI16::of(0, 20)), None);
}

#[test]
fn limits_out_of_bounds() {
    let r = RectU16::largest();
    assert_eq!(try_checked_translate(&r, &PointI16::of(i16::MIN, 0)), None);
    assert_eq!(try_checked_translate(&r, &PointI16::of(0, i16::MIN)), None);
    assert_eq!(try_checked_translate(&r, &PointI16::of(i16::MAX, 0)), None);
    assert_eq!(try_checked_translate(&r, &PointI16::of(0, i16::MAX)), None);
}

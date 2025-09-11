use super::try_checked_translate;
use crate::matrix::{point::point_i16::PointI16, rect::rect_i16::Rect};

const MIN: i16 = i16::MIN;
const MAX: i16 = i16::MAX;

#[test]
fn test() {
    assert_eq!(try_checked_translate(&Rect::of(5, 9, 13, 37), &PointI16::of(-10, -20)), Some(Rect::of(-5, -11, 3, 17)));
    assert_eq!(try_checked_translate(&Rect::of(-5, -11, 3, 17), &PointI16::of(6, -19)), Some(Rect::of(1, -30, 9, -2)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_translate(&Rect::of(MIN + 2, MIN + 5, MAX, MAX), &PointI16::of(-2, -5)), Some(Rect::of(MIN, MIN, MAX - 2, MAX - 5)));
    assert_eq!(try_checked_translate(&Rect::of(MIN, MIN, MAX - 2, MAX - 5), &PointI16::of(2, 5)), Some(Rect::of(MIN + 2, MIN + 5, MAX, MAX)));
}

#[test]
fn out_of_bounds() {
    let r = Rect::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10);
    assert_eq!(try_checked_translate(&r, &PointI16::of(-20, 0)), None);
    assert_eq!(try_checked_translate(&r, &PointI16::of(0, -20)), None);
    assert_eq!(try_checked_translate(&r, &PointI16::of(20, 0)), None);
    assert_eq!(try_checked_translate(&r, &PointI16::of(0, 20)), None);
}

#[test]
fn limits_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(try_checked_translate(&r, &PointI16::of(MIN, 0)), None);
    assert_eq!(try_checked_translate(&r, &PointI16::of(0, MIN)), None);
    assert_eq!(try_checked_translate(&r, &PointI16::of(MAX, 0)), None);
    assert_eq!(try_checked_translate(&r, &PointI16::of(0, MAX)), None);
}

use super::try_checked_translate_assign;
use crate::matrix::{point::point_i16::PointI16, rect::rect_u16::RectU16};

const MAX: u16 = u16::MAX;

#[test]
fn test() {
    let mut r = RectU16::of(0, 0, 12, 15);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI16::of(5, 4)), Some(()));
    assert_eq!(r, RectU16::of(5, 4, 17, 19));
    assert_eq!(try_checked_translate_assign(&mut r, &PointI16::of(-4, -2)), Some(()));
    assert_eq!(r, RectU16::of(1, 2, 13, 17));
}

#[test]
fn to_bounds() {
    let mut r_min = RectU16::of(2, 5, MAX, MAX);
    assert_eq!(try_checked_translate_assign(&mut r_min, &PointI16::of(-2, -5)), Some(()));
    assert_eq!(r_min, RectU16::of(0, 0, MAX - 2, MAX - 5));

    let mut r_max = RectU16::of(0, 0, MAX - 2, MAX - 5);
    assert_eq!(try_checked_translate_assign(&mut r_max, &PointI16::of(2, 5)), Some(()));
    assert_eq!(r_max, RectU16::of(2, 5, MAX, MAX));
}

#[test]
fn out_of_bounds() {
    let mut r = RectU16::of(10, 10, MAX - 10, MAX - 10);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI16::of(-20, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI16::of(0, -20)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI16::of(20, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI16::of(0, 20)), None);
    assert_eq!(r, RectU16::of(10, 10, MAX - 10, MAX - 10));
}

#[test]
fn limits_out_of_bounds() {
    let mut r = RectU16::largest();
    assert_eq!(try_checked_translate_assign(&mut r, &PointI16::of(i16::MIN, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI16::of(0, i16::MIN)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI16::of(i16::MAX, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI16::of(0, i16::MAX)), None);
    assert_eq!(r, RectU16::largest());
}

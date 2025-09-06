use super::try_checked_translate_assign;
use crate::cartesian::{point::point_i16::PointI16, rect::rect_i16::RectI16};

#[test]
fn test() {
    let mut r = RectI16::of(5, 9, 13, 37);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI16::of(-10, -20)), Some(()));
    assert_eq!(r, RectI16::of(-5, -11, 3, 17));
    assert_eq!(try_checked_translate_assign(&mut r, &PointI16::of(6, -19)), Some(()));
    assert_eq!(r, RectI16::of(1, -30, 9, -2));
}

#[test]
fn to_bounds() {
    let mut min_r = RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX);
    assert_eq!(try_checked_translate_assign(&mut min_r, &PointI16::of(-2, -5)), Some(()));
    assert_eq!(min_r, RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5));

    let mut max_r = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5);
    assert_eq!(try_checked_translate_assign(&mut max_r, &PointI16::of(2, 5)), Some(()));
    assert_eq!(max_r, RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX));
}

#[test]
fn out_of_bounds() {
    let mut r = RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 10, i16::MAX - 10);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI16::of(-20, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI16::of(0, -20)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI16::of(20, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI16::of(0, 20)), None);
    assert_eq!(r, RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 10, i16::MAX - 10));
}

#[test]
fn limits_out_of_bounds() {
    let mut r = RectI16::largest();
    assert_eq!(try_checked_translate_assign(&mut r, &PointI16::of(i16::MIN, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI16::of(0, i16::MIN)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI16::of(i16::MAX, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI16::of(0, i16::MAX)), None);
    assert_eq!(r, RectI16::largest());
}

use super::try_checked_translate_assign;
use crate::cartesian::{point::point_i8::PointI8, rect::rect_i8::RectI8};

#[test]
fn test() {
    let mut r = RectI8::of(5, 9, 13, 37);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI8::of(-10, -20)), Some(()));
    assert_eq!(r, RectI8::of(-5, -11, 3, 17));
    assert_eq!(try_checked_translate_assign(&mut r, &PointI8::of(6, -19)), Some(()));
    assert_eq!(r, RectI8::of(1, -30, 9, -2));
}

#[test]
fn to_bounds() {
    let mut r_min = RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX, i8::MAX);
    assert_eq!(try_checked_translate_assign(&mut r_min, &PointI8::of(-2, -5)), Some(()));
    assert_eq!(r_min, RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 5));

    let mut r_max = RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 5);
    assert_eq!(try_checked_translate_assign(&mut r_max, &PointI8::of(2, 5)), Some(()));
    assert_eq!(r_max, RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX, i8::MAX));
}

#[test]
fn out_of_bounds() {
    let mut r = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 10, i8::MAX - 10);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI8::of(-20, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI8::of(0, -20)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI8::of(20, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI8::of(0, 20)), None);
    assert_eq!(r, RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 10, i8::MAX - 10));
}

#[test]
fn limits_out_of_bounds() {
    let mut r = RectI8::largest();
    assert_eq!(try_checked_translate_assign(&mut r, &PointI8::of(i8::MIN, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI8::of(0, i8::MIN)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI8::of(i8::MAX, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI8::of(0, i8::MAX)), None);
    assert_eq!(r, RectI8::largest());
}

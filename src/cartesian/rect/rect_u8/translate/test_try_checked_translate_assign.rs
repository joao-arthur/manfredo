use super::try_checked_translate_assign;
use crate::cartesian::{point::point_i8::PointI8, rect::rect_u8::RectU8};

#[test]
fn test_try_checked_translate_assign() {
    let mut r = RectU8::of(0, 0, 12, 15);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI8::of(5, 4)), Some(()));
    assert_eq!(r, RectU8::of(5, 4, 17, 19));
    assert_eq!(try_checked_translate_assign(&mut r, &PointI8::of(-4, -2)), Some(()));
    assert_eq!(r, RectU8::of(1, 2, 13, 17));
}

#[test]
fn try_checked_translate_assign_to_bounds() {
    let mut min_r = RectU8::of(2, 5, u8::MAX, u8::MAX);
    assert_eq!(try_checked_translate_assign(&mut min_r, &PointI8::of(-2, -5)), Some(()));
    assert_eq!(min_r, RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 5));

    let mut max_r = RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 5);
    assert_eq!(try_checked_translate_assign(&mut max_r, &PointI8::of(2, 5)), Some(()));
    assert_eq!(max_r, RectU8::of(2, 5, u8::MAX, u8::MAX));
}

#[test]
fn try_checked_translate_assign_out_of_bounds() {
    let mut r = RectU8::of(10, 10, u8::MAX - 10, u8::MAX - 10);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI8::of(-20, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI8::of(0, -20)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI8::of(20, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI8::of(0, 20)), None);
    assert_eq!(r, RectU8::of(10, 10, u8::MAX - 10, u8::MAX - 10));
}

#[test]
fn try_checked_translate_assign_limits_out_of_bounds() {
    let mut r = RectU8::largest();
    assert_eq!(try_checked_translate_assign(&mut r, &PointI8::of(i8::MIN, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI8::of(0, i8::MIN)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI8::of(i8::MAX, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI8::of(0, i8::MAX)), None);
    assert_eq!(r, RectU8::largest());
}

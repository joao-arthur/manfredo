use super::try_checked_translate_assign;
use crate::cartesian::{point::point_i64::PointI64, rect::rect_i64::RectI64};

#[test]
fn test() {
    let mut r = RectI64::of(5, 9, 13, 37);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI64::of(-10, -20)), Some(()));
    assert_eq!(r, RectI64::of(-5, -11, 3, 17));
    assert_eq!(try_checked_translate_assign(&mut r, &PointI64::of(6, -19)), Some(()));
    assert_eq!(r, RectI64::of(1, -30, 9, -2));
}

#[test]
fn to_bounds() {
    let mut r_min = RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX, i64::MAX);
    assert_eq!(try_checked_translate_assign(&mut r_min, &PointI64::of(-2, -5)), Some(()));
    assert_eq!(r_min, RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 5));

    let mut r_max = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 5);
    assert_eq!(try_checked_translate_assign(&mut r_max, &PointI64::of(2, 5)), Some(()));
    assert_eq!(r_max, RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX, i64::MAX));
}

#[test]
fn out_of_bounds() {
    let mut r = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 10, i64::MAX - 10);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI64::of(-20, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI64::of(0, -20)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI64::of(20, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI64::of(0, 20)), None);
    assert_eq!(r, RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 10, i64::MAX - 10));
}

#[test]
fn limits_out_of_bounds() {
    let mut r = RectI64::largest();
    assert_eq!(try_checked_translate_assign(&mut r, &PointI64::of(i64::MIN, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI64::of(0, i64::MIN)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI64::of(i64::MAX, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI64::of(0, i64::MAX)), None);
    assert_eq!(r, RectI64::largest());
}

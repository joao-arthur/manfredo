use super::try_checked_translate_assign;
use crate::matrix::{point::point_i64::PointI64, rect::rect_u64::RectU64};

#[test]
fn test() {
    let mut r = RectU64::of(0, 0, 12, 15);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI64::of(5, 4)), Some(()));
    assert_eq!(r, RectU64::of(5, 4, 17, 19));
    assert_eq!(try_checked_translate_assign(&mut r, &PointI64::of(-4, -2)), Some(()));
    assert_eq!(r, RectU64::of(1, 2, 13, 17));
}

#[test]
fn to_bounds() {
    let mut r_min = RectU64::of(2, 5, u64::MAX, u64::MAX);
    assert_eq!(try_checked_translate_assign(&mut r_min, &PointI64::of(-2, -5)), Some(()));
    assert_eq!(r_min, RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5));

    let mut r_max = RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5);
    assert_eq!(try_checked_translate_assign(&mut r_max, &PointI64::of(2, 5)), Some(()));
    assert_eq!(r_max, RectU64::of(2, 5, u64::MAX, u64::MAX));
}

#[test]
fn out_of_bounds() {
    let mut r = RectU64::of(10, 10, u64::MAX - 10, u64::MAX - 10);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI64::of(-20, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI64::of(0, -20)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI64::of(20, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI64::of(0, 20)), None);
    assert_eq!(r, RectU64::of(10, 10, u64::MAX - 10, u64::MAX - 10));
}

#[test]
fn limits_out_of_bounds() {
    let mut r = RectU64::largest();
    assert_eq!(try_checked_translate_assign(&mut r, &PointI64::of(i64::MIN, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI64::of(0, i64::MIN)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI64::of(i64::MAX, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI64::of(0, i64::MAX)), None);
    assert_eq!(r, RectU64::largest());
}

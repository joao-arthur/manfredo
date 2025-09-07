use super::try_checked_translate;
use crate::matrix::{point::point_i64::PointI64, rect::rect_u64::RectU64};

const MAX: u64 = u64::MAX;

#[test]
fn test() {
    assert_eq!(try_checked_translate(&RectU64::of(0, 0, 12, 15), &PointI64::of(5, 4)), Some(RectU64::of(5, 4, 17, 19)));
    assert_eq!(try_checked_translate(&RectU64::of(5, 4, 17, 19), &PointI64::of(-4, -2)), Some(RectU64::of(1, 2, 13, 17)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_translate(&RectU64::of(2, 5, MAX, MAX), &PointI64::of(-2, -5)), Some(RectU64::of(0, 0, MAX - 2, MAX - 5)));
    assert_eq!(try_checked_translate(&RectU64::of(0, 0, MAX - 2, MAX - 5), &PointI64::of(2, 5)), Some(RectU64::of(2, 5, MAX, MAX)));
}

#[test]
fn out_of_bounds() {
    let r = RectU64::of(10, 10, MAX - 10, MAX - 10);
    assert_eq!(try_checked_translate(&r, &PointI64::of(-20, 0)), None);
    assert_eq!(try_checked_translate(&r, &PointI64::of(0, -20)), None);
    assert_eq!(try_checked_translate(&r, &PointI64::of(20, 0)), None);
    assert_eq!(try_checked_translate(&r, &PointI64::of(0, 20)), None);
}

#[test]
fn limits_out_of_bounds() {
    let r = RectU64::largest();
    assert_eq!(try_checked_translate(&r, &PointI64::of(i64::MIN, 0)), None);
    assert_eq!(try_checked_translate(&r, &PointI64::of(0, i64::MIN)), None);
    assert_eq!(try_checked_translate(&r, &PointI64::of(i64::MAX, 0)), None);
    assert_eq!(try_checked_translate(&r, &PointI64::of(0, i64::MAX)), None);
}

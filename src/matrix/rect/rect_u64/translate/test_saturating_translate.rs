use super::saturating_translate;
use crate::matrix::{point::point_i64::PointI64, rect::rect_u64::RectU64};

#[test]
fn test_saturating_translate() {
    assert_eq!(saturating_translate(&RectU64::of(0, 0, 12, 15), &PointI64::of(5, 4)), RectU64::of(5, 4, 17, 19));
    assert_eq!(saturating_translate(&RectU64::of(5, 4, 17, 19), &PointI64::of(-4, -2)), RectU64::of(1, 2, 13, 17));
}

#[test]
fn saturating_translate_to_bounds() {
    assert_eq!(saturating_translate(&RectU64::of(2, 5, u64::MAX, u64::MAX), &PointI64::of(-2, -5)), RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5));
    assert_eq!(saturating_translate(&RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5), &PointI64::of(2, 5)), RectU64::of(2, 5, u64::MAX, u64::MAX));
}

#[test]
fn saturating_translate_out_of_bounds() {
    let r = RectU64::of(10, 10, u64::MAX - 10, u64::MAX - 10);
    assert_eq!(saturating_translate(&r, &PointI64::of(-20, 0)), RectU64::of(0, 10, u64::MAX - 20, u64::MAX - 10));
    assert_eq!(saturating_translate(&r, &PointI64::of(0, -20)), RectU64::of(10, 0, u64::MAX - 10, u64::MAX - 20));
    assert_eq!(saturating_translate(&r, &PointI64::of(20, 0)), RectU64::of(20, 10, u64::MAX, u64::MAX - 10));
    assert_eq!(saturating_translate(&r, &PointI64::of(0, 20)), RectU64::of(10, 20, u64::MAX - 10, u64::MAX));
}

#[test]
fn saturating_translate_limits_out_of_bounds() {
    let r = RectU64::largest();
    assert_eq!(saturating_translate(&r, &PointI64::of(i64::MIN, 0)), RectU64::largest());
    assert_eq!(saturating_translate(&r, &PointI64::of(0, i64::MIN)), RectU64::largest());
    assert_eq!(saturating_translate(&r, &PointI64::of(i64::MAX, 0)), RectU64::largest());
    assert_eq!(saturating_translate(&r, &PointI64::of(0, i64::MAX)), RectU64::largest());
}

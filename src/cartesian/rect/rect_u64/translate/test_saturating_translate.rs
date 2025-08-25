use super::saturating_translate;
use crate::cartesian::{point::point_i64::PointI64, rect::rect_u64::RectU64};

#[test]
fn test_saturating_translate() {
    assert_eq!(saturating_translate(&RectU64::of(0, 0, 12, 15), &PointI64::of(5, 4)), RectU64::of(5, 4, 17, 19));
    assert_eq!(saturating_translate(&RectU64::of(5, 4, 17, 19), &PointI64::of(-4, -2)), RectU64::of(1, 2, 13, 17));
}

#[test]
fn saturating_translate_small_rect_to_bounds() {
    assert_eq!(saturating_translate(&RectU64::of(2, 5, 12, 15), &PointI64::of(-2, -5)), RectU64::of(0, 0, 10, 10));
    assert_eq!(
        saturating_translate(&RectU64::of(u64::MAX - 12, u64::MAX - 15, u64::MAX - 2, u64::MAX - 5), &PointI64::of(2, 5)),
        RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX, u64::MAX)
    );
}

#[test]
fn saturating_translate_big_rect_to_bounds() {
    assert_eq!(saturating_translate(&RectU64::of(2, 5, u64::MAX, u64::MAX), &PointI64::of(-2, -5)), RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5));
    assert_eq!(saturating_translate(&RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5), &PointI64::of(2, 5)), RectU64::of(2, 5, u64::MAX, u64::MAX));
}

#[test]
fn saturating_translate_small_rect_out_of_bounds() {
    assert_eq!(saturating_translate(&RectU64::of(10, 5, 20, 30), &PointI64::of(-20, -20)), RectU64::of(0, 0, 10, 25));
    assert_eq!(
        saturating_translate(&RectU64::of(u64::MAX - 20, u64::MAX - 30, u64::MAX - 5, u64::MAX - 10), &PointI64::of(20, 20)),
        RectU64::of(u64::MAX - 15, u64::MAX - 20, u64::MAX, u64::MAX)
    );
}

#[test]
fn saturating_translate_big_rect_out_of_bounds() {
    assert_eq!(
        saturating_translate(&RectU64::of(10, 5, u64::MAX, u64::MAX), &PointI64::of(-20, -20)),
        RectU64::of(0, 0, u64::MAX - 10, u64::MAX - 5)
    );
    assert_eq!(saturating_translate(&RectU64::of(0, 0, u64::MAX - 5, u64::MAX - 10), &PointI64::of(20, 20)), RectU64::of(5, 10, u64::MAX, u64::MAX));
}

#[test]
fn saturating_translate_small_rect_limits_out_of_bounds() {
    assert_eq!(saturating_translate(&RectU64::of(1, 1, 10, 10), &PointI64::min()), RectU64::of(0, 0, 9, 9));
    assert_eq!(
        saturating_translate(&RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX - 1, u64::MAX - 1), &PointI64::max()),
        RectU64::of(u64::MAX - 9, u64::MAX - 9, u64::MAX, u64::MAX)
    );
}

#[test]
fn saturating_translate_big_rect_limits_out_of_bounds() {
    assert_eq!(saturating_translate(&RectU64::of(1, 1, u64::MAX, u64::MAX), &PointI64::min()), RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1));
    assert_eq!(saturating_translate(&RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1), &PointI64::max()), RectU64::of(1, 1, u64::MAX, u64::MAX));
}

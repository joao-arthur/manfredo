
use super::try_checked_add;
use crate::cartesian::rect::{rect_i64::RectI64, rect_u64::RectU64};

#[test]
fn test_try_checked_add() {
    assert_eq!(try_checked_add(&RectU64::of(0, 0, 12, 15), &RectI64::of(5, 4, 3, 2)), Some(RectU64::of(5, 4, 15, 17)));
    assert_eq!(try_checked_add(&RectU64::of(5, 4, 15, 20), &RectI64::of(-4, -3, -2, -1)), Some(RectU64::of(1, 1, 13, 19)));
}

#[test]
fn try_checked_add_to_bounds() {
    assert_eq!(try_checked_add(&RectU64::of(2, 5, u64::MAX - 2, u64::MAX - 5), &RectI64::of(-2, -5, 2, 5)), Some(RectU64::largest()));
    assert_eq!(try_checked_add(&RectU64::of(2, 5, u64::MAX, u64::MAX), &RectI64::of(-2, -5, 0, 0)), Some(RectU64::largest()));
    assert_eq!(try_checked_add(&RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5), &RectI64::of(0, 0, 2, 5)), Some(RectU64::largest()));
}

#[test]
fn try_checked_add_edge_out_of_bounds() {
    let r = RectU64::largest();
    assert_eq!(try_checked_add(&r, &RectI64::of(-1, 0, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI64::of(0, -1, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI64::of(0, 0, 1, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI64::of(0, 0, 0, 1)), None);
}

#[test]
fn try_checked_add_out_of_bounds() {
    let r = RectU64::of(10, 10, u64::MAX - 10, u64::MAX - 10);
    assert_eq!(try_checked_add(&r, &RectI64::of(-20, 0, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI64::of(0, -20, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI64::of(0, 0, 20, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI64::of(0, 0, 0, 20)), None);
}

#[test]
fn try_checked_add_limits_out_of_bounds() {
    let r = RectU64::largest();
    assert_eq!(try_checked_add(&r, &RectI64::of(i64::MIN, 0, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI64::of(0, i64::MIN, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI64::of(0, 0, i64::MAX, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI64::of(0, 0, 0, i64::MAX)), None);
}

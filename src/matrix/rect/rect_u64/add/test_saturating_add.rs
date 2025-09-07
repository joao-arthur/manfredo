use super::saturating_add;
use crate::matrix::rect::{rect_i64::RectI64, rect_u64::RectU64};

const MAX: u64 = u64::MAX;

#[test]
fn test() {
    assert_eq!(saturating_add(&RectU64::of(0, 0, 12, 15), &RectI64::of(5, 4, 3, 2)), RectU64::of(5, 4, 15, 17));
    assert_eq!(saturating_add(&RectU64::of(5, 4, 15, 20), &RectI64::of(-4, -3, -2, -1)), RectU64::of(1, 1, 13, 19));
}

#[test]
fn to_bounds() {
    assert_eq!(saturating_add(&RectU64::of(2, 5, MAX - 2, MAX - 5), &RectI64::of(-2, -5, 2, 5)), RectU64::largest());
    assert_eq!(saturating_add(&RectU64::of(2, 5, MAX, MAX), &RectI64::of(-2, -5, 0, 0)), RectU64::largest());
    assert_eq!(saturating_add(&RectU64::of(0, 0, MAX - 2, MAX - 5), &RectI64::of(0, 0, 2, 5)), RectU64::largest());
}

#[test]
fn out_of_bounds() {
    let r = RectU64::of(10, 10, MAX - 10, MAX - 10);
    assert_eq!(saturating_add(&r, &RectI64::of(-20, 0, 0, 0)), RectU64::of(0, 10, MAX - 10, MAX - 10));
    assert_eq!(saturating_add(&r, &RectI64::of(0, -20, 0, 0)), RectU64::of(10, 0, MAX - 10, MAX - 10));
    assert_eq!(saturating_add(&r, &RectI64::of(0, 0, 20, 0)), RectU64::of(10, 10, MAX, MAX - 10));
    assert_eq!(saturating_add(&r, &RectI64::of(0, 0, 0, 20)), RectU64::of(10, 10, MAX - 10, MAX));
}

#[test]
fn edge_out_of_bounds() {
    let r = RectU64::largest();
    assert_eq!(saturating_add(&r, &RectI64::of(-1, 0, 0, 0)), RectU64::largest());
    assert_eq!(saturating_add(&r, &RectI64::of(0, -1, 0, 0)), RectU64::largest());
    assert_eq!(saturating_add(&r, &RectI64::of(0, 0, 1, 0)), RectU64::largest());
    assert_eq!(saturating_add(&r, &RectI64::of(0, 0, 0, 1)), RectU64::largest());
}

#[test]
fn limits_out_of_bounds() {
    let r = RectU64::largest();
    assert_eq!(saturating_add(&r, &RectI64::of(i64::MIN, 0, 0, 0)), RectU64::largest());
    assert_eq!(saturating_add(&r, &RectI64::of(0, i64::MIN, 0, 0)), RectU64::largest());
    assert_eq!(saturating_add(&r, &RectI64::of(0, 0, i64::MAX, 0)), RectU64::largest());
    assert_eq!(saturating_add(&r, &RectI64::of(0, 0, 0, i64::MAX)), RectU64::largest());
}

use super::wrapping_add;
use crate::cartesian::rect::{rect_i64::Rect, rect_u64::RectU64};

const MAX: u64 = u64::MAX;

#[test]
fn test() {
    assert_eq!(wrapping_add(&RectU64::of(0, 0, 12, 10), &Rect::of(5, 4, 3, 2)), RectU64::of(5, 4, 15, 12));
    assert_eq!(wrapping_add(&RectU64::of(5, 4, 15, 12), &Rect::of(-4, -3, -2, -1)), RectU64::of(1, 1, 13, 11));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_add(&RectU64::of(2, 5, MAX - 2, MAX - 5), &Rect::of(-2, -5, 2, 5)), RectU64::largest());
    assert_eq!(wrapping_add(&RectU64::of(2, 5, MAX, MAX), &Rect::of(-2, -5, 0, 0)), RectU64::largest());
    assert_eq!(wrapping_add(&RectU64::of(0, 0, MAX - 2, MAX - 5), &Rect::of(0, 0, 2, 5)), RectU64::largest());
}

#[test]
fn out_of_bounds() {
    let r = RectU64::of(10, 10, MAX - 10, MAX - 10);
    assert_eq!(wrapping_add(&r, &Rect::of(-20, 0, 0, 0)), RectU64::of(MAX - 9, 10, MAX - 10, MAX - 10));
    assert_eq!(wrapping_add(&r, &Rect::of(0, -20, 0, 0)), RectU64::of(10, MAX - 9, MAX - 10, MAX - 10));
    assert_eq!(wrapping_add(&r, &Rect::of(0, 0, 20, 0)), RectU64::of(10, 10, 9, MAX - 10));
    assert_eq!(wrapping_add(&r, &Rect::of(0, 0, 0, 20)), RectU64::of(10, 10, MAX - 10, 9));
}

#[test]
fn edge_out_of_bounds() {
    let r = RectU64::largest();
    assert_eq!(wrapping_add(&r, &Rect::of(-1, 0, 0, 0)), RectU64::of(MAX, 0, MAX, MAX));
    assert_eq!(wrapping_add(&r, &Rect::of(0, -1, 0, 0)), RectU64::of(0, MAX, MAX, MAX));
    assert_eq!(wrapping_add(&r, &Rect::of(0, 0, 1, 0)), RectU64::of(0, 0, 0, MAX));
    assert_eq!(wrapping_add(&r, &Rect::of(0, 0, 0, 1)), RectU64::of(0, 0, MAX, 0));
}

#[test]
fn limits_out_of_bounds() {
    let r = RectU64::largest();
    assert_eq!(wrapping_add(&r, &Rect::of(i64::MIN, 0, 0, 0)), RectU64::of(MAX / 2 + 1, 0, MAX, MAX));
    assert_eq!(wrapping_add(&r, &Rect::of(0, i64::MIN, 0, 0)), RectU64::of(0, MAX / 2 + 1, MAX, MAX));
    assert_eq!(wrapping_add(&r, &Rect::of(0, 0, i64::MAX, 0)), RectU64::of(0, 0, MAX / 2 - 1, MAX));
    assert_eq!(wrapping_add(&r, &Rect::of(0, 0, 0, i64::MAX)), RectU64::of(0, 0, MAX, MAX / 2 - 1));
}

use super::try_checked_add;
use crate::matrix::rect::{rect_i64::Rect, rect_u64::RectU64};

const MAX: u64 = u64::MAX;

#[test]
fn test() {
    assert_eq!(try_checked_add(&RectU64::of(0, 0, 12, 15), &Rect::of(5, 4, 3, 2)), Some(RectU64::of(5, 4, 15, 17)));
    assert_eq!(try_checked_add(&RectU64::of(5, 4, 15, 20), &Rect::of(-4, -3, -2, -1)), Some(RectU64::of(1, 1, 13, 19)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_add(&RectU64::of(2, 5, MAX - 2, MAX - 5), &Rect::of(-2, -5, 2, 5)), Some(RectU64::largest()));
    assert_eq!(try_checked_add(&RectU64::of(2, 5, MAX, MAX), &Rect::of(-2, -5, 0, 0)), Some(RectU64::largest()));
    assert_eq!(try_checked_add(&RectU64::of(0, 0, MAX - 2, MAX - 5), &Rect::of(0, 0, 2, 5)), Some(RectU64::largest()));
}

#[test]
fn out_of_bounds() {
    let r = RectU64::of(10, 10, MAX - 10, MAX - 10);
    assert_eq!(try_checked_add(&r, &Rect::of(-20, 0, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &Rect::of(0, -20, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &Rect::of(0, 0, 20, 0)), None);
    assert_eq!(try_checked_add(&r, &Rect::of(0, 0, 0, 20)), None);
}

#[test]
fn edge_out_of_bounds() {
    let r = RectU64::largest();
    assert_eq!(try_checked_add(&r, &Rect::of(-1, 0, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &Rect::of(0, -1, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &Rect::of(0, 0, 1, 0)), None);
    assert_eq!(try_checked_add(&r, &Rect::of(0, 0, 0, 1)), None);
}

#[test]
fn limits_out_of_bounds() {
    let r = RectU64::largest();
    assert_eq!(try_checked_add(&r, &Rect::of(i64::MIN, 0, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &Rect::of(0, i64::MIN, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &Rect::of(0, 0, i64::MAX, 0)), None);
    assert_eq!(try_checked_add(&r, &Rect::of(0, 0, 0, i64::MAX)), None);
}

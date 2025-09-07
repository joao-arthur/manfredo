use super::try_checked_resize;
use crate::cartesian::rect::rect_u64::RectU64;

const MAX: u64 = u64::MAX;

#[test]
fn odd() {
    assert_eq!(try_checked_resize(&RectU64::of(5, 5, 15, 15), 9), Some(RectU64::of(6, 6, 14, 14)));
    assert_eq!(try_checked_resize(&RectU64::of(6, 6, 14, 14), 7), Some(RectU64::of(7, 7, 13, 13)));
    assert_eq!(try_checked_resize(&RectU64::of(7, 7, 13, 13), 5), Some(RectU64::of(8, 8, 12, 12)));
    assert_eq!(try_checked_resize(&RectU64::of(8, 8, 12, 12), 3), Some(RectU64::of(9, 9, 11, 11)));
    assert_eq!(try_checked_resize(&RectU64::of(9, 9, 11, 11), 9), Some(RectU64::of(6, 6, 14, 14)));
}

#[test]
fn even() {
    assert_eq!(try_checked_resize(&RectU64::of(5, 5, 14, 14), 10), Some(RectU64::of(5, 5, 14, 14)));
    assert_eq!(try_checked_resize(&RectU64::of(5, 5, 14, 14), 8), Some(RectU64::of(6, 6, 13, 13)));
    assert_eq!(try_checked_resize(&RectU64::of(6, 6, 13, 13), 6), Some(RectU64::of(7, 7, 12, 12)));
    assert_eq!(try_checked_resize(&RectU64::of(7, 7, 12, 12), 4), Some(RectU64::of(8, 8, 11, 11)));
    assert_eq!(try_checked_resize(&RectU64::of(8, 8, 11, 11), 8), Some(RectU64::of(6, 6, 13, 13)));
}

#[test]
fn small_size() {
    let r = RectU64::of(10, 10, 20, 20);
    assert_eq!(try_checked_resize(&r, 0), None);
    assert_eq!(try_checked_resize(&r, 1), None);
    assert_eq!(try_checked_resize(&r, 2), None);
}

#[test]
fn same_size() {
    assert_eq!(try_checked_resize(&RectU64::of(0, 0, 2, 2), 3), Some(RectU64::of(0, 0, 2, 2)));
    assert_eq!(try_checked_resize(&RectU64::of(0, 0, 3, 3), 4), Some(RectU64::of(0, 0, 3, 3)));
    assert_eq!(try_checked_resize(&RectU64::of(MAX - 2, MAX - 2, MAX, MAX), 3), Some(RectU64::of(MAX - 2, MAX - 2, MAX, MAX)));
    assert_eq!(try_checked_resize(&RectU64::of(MAX - 3, MAX - 3, MAX, MAX), 4), Some(RectU64::of(MAX - 3, MAX - 3, MAX, MAX)));
}

#[test]
fn out_of_bounds() {
    assert_eq!(try_checked_resize(&RectU64::of(0, 2, 2, 4), 5), None);
    assert_eq!(try_checked_resize(&RectU64::of(2, 0, 4, 2), 5), None);
    assert_eq!(try_checked_resize(&RectU64::of(MAX - 2, MAX - 4, MAX, MAX - 2), 5), None);
    assert_eq!(try_checked_resize(&RectU64::of(MAX - 4, MAX - 2, MAX - 2, MAX), 5), None);
}

#[test]
fn small_rect_limits_out_of_bounds() {
    assert_eq!(try_checked_resize(&RectU64::of(0, 2, 2, 4), MAX), None);
    assert_eq!(try_checked_resize(&RectU64::of(2, 0, 4, 2), MAX), None);
    assert_eq!(try_checked_resize(&RectU64::of(MAX - 2, MAX - 4, MAX, MAX - 2), MAX), None);
    assert_eq!(try_checked_resize(&RectU64::of(MAX - 4, MAX - 2, MAX - 2, MAX), MAX), None);
}

#[test]
fn big_rect_limits_out_of_bounds() {
    assert_eq!(try_checked_resize(&RectU64::of(0, 0, MAX - 1, MAX - 1), MAX), Some(RectU64::of(0, 0, MAX - 1, MAX - 1)));
    assert_eq!(try_checked_resize(&RectU64::of(1, 1, MAX, MAX), MAX), Some(RectU64::of(1, 1, MAX, MAX)));
    assert_eq!(try_checked_resize(&RectU64::largest(), MAX), Some(RectU64::of(0, 0, MAX - 1, MAX - 1)));
}

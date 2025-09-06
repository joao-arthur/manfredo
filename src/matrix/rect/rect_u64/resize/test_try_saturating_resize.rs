use super::try_saturating_resize;
use crate::matrix::rect::rect_u64::RectU64;

#[test]
fn try_saturating_resize_odd() {
    assert_eq!(try_saturating_resize(&RectU64::of(5, 5, 15, 15), 9), Some(RectU64::of(6, 6, 14, 14)));
    assert_eq!(try_saturating_resize(&RectU64::of(6, 6, 14, 14), 7), Some(RectU64::of(7, 7, 13, 13)));
    assert_eq!(try_saturating_resize(&RectU64::of(7, 7, 13, 13), 5), Some(RectU64::of(8, 8, 12, 12)));
    assert_eq!(try_saturating_resize(&RectU64::of(8, 8, 12, 12), 3), Some(RectU64::of(9, 9, 11, 11)));
    assert_eq!(try_saturating_resize(&RectU64::of(9, 9, 11, 11), 9), Some(RectU64::of(6, 6, 14, 14)));
}

#[test]
fn try_saturating_resize_even() {
    assert_eq!(try_saturating_resize(&RectU64::of(5, 5, 14, 14), 10), Some(RectU64::of(5, 5, 14, 14)));
    assert_eq!(try_saturating_resize(&RectU64::of(5, 5, 14, 14), 8), Some(RectU64::of(6, 6, 13, 13)));
    assert_eq!(try_saturating_resize(&RectU64::of(6, 6, 13, 13), 6), Some(RectU64::of(7, 7, 12, 12)));
    assert_eq!(try_saturating_resize(&RectU64::of(7, 7, 12, 12), 4), Some(RectU64::of(8, 8, 11, 11)));
    assert_eq!(try_saturating_resize(&RectU64::of(8, 8, 11, 11), 8), Some(RectU64::of(6, 6, 13, 13)));
}

#[test]
fn try_saturating_resize_small_size() {
    let r = RectU64::of(10, 10, 20, 20);
    assert_eq!(try_saturating_resize(&r, 0), None);
    assert_eq!(try_saturating_resize(&r, 1), None);
    assert_eq!(try_saturating_resize(&r, 2), None);
}

#[test]
fn try_saturating_resize_same_size() {
    assert_eq!(try_saturating_resize(&RectU64::of(0, 0, 2, 2), 3), Some(RectU64::of(0, 0, 2, 2)));
    assert_eq!(try_saturating_resize(&RectU64::of(0, 0, 3, 3), 4), Some(RectU64::of(0, 0, 3, 3)));
    assert_eq!(try_saturating_resize(&RectU64::of(u64::MAX - 2, u64::MAX - 2, u64::MAX, u64::MAX), 3), Some(RectU64::of(u64::MAX - 2, u64::MAX - 2, u64::MAX, u64::MAX)));
    assert_eq!(try_saturating_resize(&RectU64::of(u64::MAX - 3, u64::MAX - 3, u64::MAX, u64::MAX), 4), Some(RectU64::of(u64::MAX - 3, u64::MAX - 3, u64::MAX, u64::MAX)));
}

#[test]
fn try_saturating_resize_bounds() {
    assert_eq!(try_saturating_resize(&RectU64::of(0, 0, 2, 2), 11), Some(RectU64::of(0, 0, 10, 10)));
    assert_eq!(try_saturating_resize(&RectU64::of(u64::MAX - 2, u64::MAX - 2, u64::MAX, u64::MAX), 11), Some(RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX, u64::MAX)));
}

#[test]
fn try_saturating_resize_small_rect_limits() {
    assert_eq!(try_saturating_resize(&RectU64::of(0, 0, 2, 2), u64::MAX), Some(RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1)));
    assert_eq!(try_saturating_resize(&RectU64::of(u64::MAX - 2, u64::MAX - 2, u64::MAX, u64::MAX), u64::MAX), Some(RectU64::of(1, 1, u64::MAX, u64::MAX)));
}

#[test]
fn try_saturating_resize_big_rect_limits() {
    assert_eq!(try_saturating_resize(&RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1), u64::MAX), Some(RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1)));
    assert_eq!(try_saturating_resize(&RectU64::of(1, 1, u64::MAX, u64::MAX), u64::MAX), Some(RectU64::of(1, 1, u64::MAX, u64::MAX)));
    assert_eq!(try_saturating_resize(&RectU64::largest(), u64::MAX), Some(RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1)));
}

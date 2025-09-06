use super::try_wrapping_resize;
use crate::cartesian::rect::rect_i64::RectI64;

#[test]
fn try_wrapping_resize_odd() {
    assert_eq!(try_wrapping_resize(&RectI64::of(-5, -5, 5, 5), 9), Some(RectI64::of(-4, -4, 4, 4)));
    assert_eq!(try_wrapping_resize(&RectI64::of(-4, -4, 4, 4), 7), Some(RectI64::of(-3, -3, 3, 3)));
    assert_eq!(try_wrapping_resize(&RectI64::of(-3, -3, 3, 3), 5), Some(RectI64::of(-2, -2, 2, 2)));
    assert_eq!(try_wrapping_resize(&RectI64::of(-2, -2, 2, 2), 3), Some(RectI64::of(-1, -1, 1, 1)));
    assert_eq!(try_wrapping_resize(&RectI64::of(-1, -1, 1, 1), 9), Some(RectI64::of(-4, -4, 4, 4)));
}

#[test]
fn try_wrapping_resize_even() {
    assert_eq!(try_wrapping_resize(&RectI64::of(-5, -5, 4, 4), 10), Some(RectI64::of(-5, -5, 4, 4)));
    assert_eq!(try_wrapping_resize(&RectI64::of(-5, -5, 4, 4), 8), Some(RectI64::of(-4, -4, 3, 3)));
    assert_eq!(try_wrapping_resize(&RectI64::of(-4, -4, 3, 3), 6), Some(RectI64::of(-3, -3, 2, 2)));
    assert_eq!(try_wrapping_resize(&RectI64::of(-3, -3, 2, 2), 4), Some(RectI64::of(-2, -2, 1, 1)));
    assert_eq!(try_wrapping_resize(&RectI64::of(-2, -2, 1, 1), 8), Some(RectI64::of(-4, -4, 3, 3)));
}

#[test]
fn try_wrapping_resize_small_size() {
    let r = RectI64::of(10, 10, 20, 20);
    assert_eq!(try_wrapping_resize(&r, 0), None);
    assert_eq!(try_wrapping_resize(&r, 1), None);
    assert_eq!(try_wrapping_resize(&r, 2), None);
}

#[test]
fn try_wrapping_resize_same_size() {
    assert_eq!(try_wrapping_resize(&RectI64::of(0, 0, 2, 2), 3), Some(RectI64::of(0, 0, 2, 2)));
    assert_eq!(try_wrapping_resize(&RectI64::of(0, 0, 3, 3), 4), Some(RectI64::of(0, 0, 3, 3)));
    assert_eq!(try_wrapping_resize(&RectI64::of(i64::MAX - 2, i64::MAX - 2, i64::MAX, i64::MAX), 3), Some(RectI64::of(i64::MAX - 2, i64::MAX - 2, i64::MAX, i64::MAX)));
    assert_eq!(try_wrapping_resize(&RectI64::of(i64::MAX - 3, i64::MAX - 3, i64::MAX, i64::MAX), 4), Some(RectI64::of(i64::MAX - 3, i64::MAX - 3, i64::MAX, i64::MAX)));
}

#[test]
fn try_wrapping_resize_small_rect_out_of_bounds() {
    assert_eq!(try_wrapping_resize(&RectI64::of(0, 2, 2, 4), 5), Some(RectI64::of(i64::MAX, 1, 3, 5)));
    assert_eq!(try_wrapping_resize(&RectI64::of(2, 0, 4, 2), 5), Some(RectI64::of(1, i64::MAX, 5, 3)));
    assert_eq!(try_wrapping_resize(&RectI64::of(i64::MAX - 2, i64::MAX - 4, i64::MAX, i64::MAX - 2), 5), Some(RectI64::of(i64::MAX - 3, i64::MAX - 5, 0, i64::MAX - 1)));
    assert_eq!(try_wrapping_resize(&RectI64::of(i64::MAX - 4, i64::MAX - 2, i64::MAX - 2, i64::MAX), 5), Some(RectI64::of(i64::MAX - 5, i64::MAX - 3, i64::MAX - 1, 0)));
}

#[test]
fn try_wrapping_resize_small_rect_limits_out_of_bounds() {
    assert_eq!(try_wrapping_resize(&RectI64::of(0, 2, 2, 4), u64::MAX), Some(RectI64::of(i64::MAX / 2 + 3, i64::MAX / 2 + 5, i64::MAX / 2 + 1, i64::MAX / 2 + 3)));
    assert_eq!(try_wrapping_resize(&RectI64::of(2, 0, 4, 2), u64::MAX), Some(RectI64::of(i64::MAX / 2 + 5, i64::MAX / 2 + 3, i64::MAX / 2 + 3, i64::MAX / 2 + 1)));
    assert_eq!(try_wrapping_resize(&RectI64::of(i64::MAX - 2, i64::MAX - 4, i64::MAX, i64::MAX - 2), u64::MAX), Some(RectI64::of(i64::MAX / 2, i64::MAX / 2 - 2, i64::MAX / 2 - 2, i64::MAX / 2 - 4)));
    assert_eq!(try_wrapping_resize(&RectI64::of(i64::MAX - 4, i64::MAX - 2, i64::MAX - 2, i64::MAX), u64::MAX), Some(RectI64::of(i64::MAX / 2 - 2, i64::MAX / 2, i64::MAX / 2 - 4, i64::MAX / 2 - 2)));
}

#[test]
fn try_wrapping_resize_big_rect_limits_out_of_bounds() {
    assert_eq!(try_wrapping_resize(&RectI64::of(0, 0, i64::MAX - 1, i64::MAX - 1), u64::MAX), Some(RectI64::of(0, 0, i64::MAX - 1, i64::MAX - 1)));
    assert_eq!(try_wrapping_resize(&RectI64::of(1, 1, i64::MAX, i64::MAX), u64::MAX), Some(RectI64::of(1, 1, i64::MAX, i64::MAX)));
    assert_eq!(try_wrapping_resize(&RectI64::largest(), u64::MAX), Some(RectI64::of(0, 0, i64::MAX - 1, i64::MAX - 1)));
}

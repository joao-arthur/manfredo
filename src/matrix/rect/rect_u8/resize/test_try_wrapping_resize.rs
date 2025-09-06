use super::try_wrapping_resize;
use crate::matrix::rect::rect_u8::RectU8;

#[test]
fn try_wrapping_resize_odd() {
    assert_eq!(try_wrapping_resize(&RectU8::of(5, 5, 15, 15), 9), Some(RectU8::of(6, 6, 14, 14)));
    assert_eq!(try_wrapping_resize(&RectU8::of(6, 6, 14, 14), 7), Some(RectU8::of(7, 7, 13, 13)));
    assert_eq!(try_wrapping_resize(&RectU8::of(7, 7, 13, 13), 5), Some(RectU8::of(8, 8, 12, 12)));
    assert_eq!(try_wrapping_resize(&RectU8::of(8, 8, 12, 12), 3), Some(RectU8::of(9, 9, 11, 11)));
    assert_eq!(try_wrapping_resize(&RectU8::of(9, 9, 11, 11), 9), Some(RectU8::of(6, 6, 14, 14)));
}

#[test]
fn try_wrapping_resize_even() {
    assert_eq!(try_wrapping_resize(&RectU8::of(5, 5, 14, 14), 10), Some(RectU8::of(5, 5, 14, 14)));
    assert_eq!(try_wrapping_resize(&RectU8::of(5, 5, 14, 14), 8), Some(RectU8::of(6, 6, 13, 13)));
    assert_eq!(try_wrapping_resize(&RectU8::of(6, 6, 13, 13), 6), Some(RectU8::of(7, 7, 12, 12)));
    assert_eq!(try_wrapping_resize(&RectU8::of(7, 7, 12, 12), 4), Some(RectU8::of(8, 8, 11, 11)));
    assert_eq!(try_wrapping_resize(&RectU8::of(8, 8, 11, 11), 8), Some(RectU8::of(6, 6, 13, 13)));
}

#[test]
fn try_wrapping_resize_small_size() {
    let r = RectU8::of(10, 10, 20, 20);
    assert_eq!(try_wrapping_resize(&r, 0), None);
    assert_eq!(try_wrapping_resize(&r, 1), None);
    assert_eq!(try_wrapping_resize(&r, 2), None);
}

#[test]
fn try_wrapping_resize_same_size() {
    assert_eq!(try_wrapping_resize(&RectU8::of(0, 0, 2, 2), 3), Some(RectU8::of(0, 0, 2, 2)));
    assert_eq!(try_wrapping_resize(&RectU8::of(0, 0, 3, 3), 4), Some(RectU8::of(0, 0, 3, 3)));
    assert_eq!(try_wrapping_resize(&RectU8::of(u8::MAX - 2, u8::MAX - 2, u8::MAX, u8::MAX), 3), Some(RectU8::of(u8::MAX - 2, u8::MAX - 2, u8::MAX, u8::MAX)));
    assert_eq!(try_wrapping_resize(&RectU8::of(u8::MAX - 3, u8::MAX - 3, u8::MAX, u8::MAX), 4), Some(RectU8::of(u8::MAX - 3, u8::MAX - 3, u8::MAX, u8::MAX)));
}

#[test]
fn try_wrapping_resize_small_rect_out_of_bounds() {
    assert_eq!(try_wrapping_resize(&RectU8::of(0, 2, 2, 4), 5), Some(RectU8::of(u8::MAX, 1, 3, 5)));
    assert_eq!(try_wrapping_resize(&RectU8::of(2, 0, 4, 2), 5), Some(RectU8::of(1, u8::MAX, 5, 3)));
    assert_eq!(try_wrapping_resize(&RectU8::of(u8::MAX - 2, u8::MAX - 4, u8::MAX, u8::MAX - 2), 5), Some(RectU8::of(u8::MAX - 3, u8::MAX - 5, 0, u8::MAX - 1)));
    assert_eq!(try_wrapping_resize(&RectU8::of(u8::MAX - 4, u8::MAX - 2, u8::MAX - 2, u8::MAX), 5), Some(RectU8::of(u8::MAX - 5, u8::MAX - 3, u8::MAX - 1, 0)));
}

#[test]
fn try_wrapping_resize_small_rect_limits_out_of_bounds() {
    assert_eq!(try_wrapping_resize(&RectU8::of(0, 2, 2, 4), u8::MAX), Some(RectU8::of(u8::MAX / 2 + 3, u8::MAX / 2 + 5, u8::MAX / 2 + 1, u8::MAX / 2 + 3)));
    assert_eq!(try_wrapping_resize(&RectU8::of(2, 0, 4, 2), u8::MAX), Some(RectU8::of(u8::MAX / 2 + 5, u8::MAX / 2 + 3, u8::MAX / 2 + 3, u8::MAX / 2 + 1)));
    assert_eq!(try_wrapping_resize(&RectU8::of(u8::MAX - 2, u8::MAX - 4, u8::MAX, u8::MAX - 2), u8::MAX), Some(RectU8::of(u8::MAX / 2, u8::MAX / 2 - 2, u8::MAX / 2 - 2, u8::MAX / 2 - 4)));
    assert_eq!(try_wrapping_resize(&RectU8::of(u8::MAX - 4, u8::MAX - 2, u8::MAX - 2, u8::MAX), u8::MAX), Some(RectU8::of(u8::MAX / 2 - 2, u8::MAX / 2, u8::MAX / 2 - 4, u8::MAX / 2 - 2)));
}

#[test]
fn try_wrapping_resize_big_rect_limits_out_of_bounds() {
    assert_eq!(try_wrapping_resize(&RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1), u8::MAX), Some(RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1)));
    assert_eq!(try_wrapping_resize(&RectU8::of(1, 1, u8::MAX, u8::MAX), u8::MAX), Some(RectU8::of(1, 1, u8::MAX, u8::MAX)));
    assert_eq!(try_wrapping_resize(&RectU8::largest(), u8::MAX), Some(RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1)));
}

use super::try_wrapping_resize;
use crate::matrix::rect::rect_u32::RectU32;

#[test]
fn odd() {
    assert_eq!(try_wrapping_resize(&RectU32::of(5, 5, 15, 15), 9), Some(RectU32::of(6, 6, 14, 14)));
    assert_eq!(try_wrapping_resize(&RectU32::of(6, 6, 14, 14), 7), Some(RectU32::of(7, 7, 13, 13)));
    assert_eq!(try_wrapping_resize(&RectU32::of(7, 7, 13, 13), 5), Some(RectU32::of(8, 8, 12, 12)));
    assert_eq!(try_wrapping_resize(&RectU32::of(8, 8, 12, 12), 3), Some(RectU32::of(9, 9, 11, 11)));
    assert_eq!(try_wrapping_resize(&RectU32::of(9, 9, 11, 11), 9), Some(RectU32::of(6, 6, 14, 14)));
}

#[test]
fn even() {
    assert_eq!(try_wrapping_resize(&RectU32::of(5, 5, 14, 14), 10), Some(RectU32::of(5, 5, 14, 14)));
    assert_eq!(try_wrapping_resize(&RectU32::of(5, 5, 14, 14), 8), Some(RectU32::of(6, 6, 13, 13)));
    assert_eq!(try_wrapping_resize(&RectU32::of(6, 6, 13, 13), 6), Some(RectU32::of(7, 7, 12, 12)));
    assert_eq!(try_wrapping_resize(&RectU32::of(7, 7, 12, 12), 4), Some(RectU32::of(8, 8, 11, 11)));
    assert_eq!(try_wrapping_resize(&RectU32::of(8, 8, 11, 11), 8), Some(RectU32::of(6, 6, 13, 13)));
}

#[test]
fn small_size() {
    let r = RectU32::of(10, 10, 20, 20);
    assert_eq!(try_wrapping_resize(&r, 0), None);
    assert_eq!(try_wrapping_resize(&r, 1), None);
    assert_eq!(try_wrapping_resize(&r, 2), None);
}

#[test]
fn same_size() {
    assert_eq!(try_wrapping_resize(&RectU32::of(0, 0, 2, 2), 3), Some(RectU32::of(0, 0, 2, 2)));
    assert_eq!(try_wrapping_resize(&RectU32::of(0, 0, 3, 3), 4), Some(RectU32::of(0, 0, 3, 3)));
    assert_eq!(try_wrapping_resize(&RectU32::of(u32::MAX - 2, u32::MAX - 2, u32::MAX, u32::MAX), 3), Some(RectU32::of(u32::MAX - 2, u32::MAX - 2, u32::MAX, u32::MAX)));
    assert_eq!(try_wrapping_resize(&RectU32::of(u32::MAX - 3, u32::MAX - 3, u32::MAX, u32::MAX), 4), Some(RectU32::of(u32::MAX - 3, u32::MAX - 3, u32::MAX, u32::MAX)));
}

#[test]
fn out_of_bounds() {
    assert_eq!(try_wrapping_resize(&RectU32::of(0, 2, 2, 4), 5), Some(RectU32::of(u32::MAX, 1, 3, 5)));
    assert_eq!(try_wrapping_resize(&RectU32::of(2, 0, 4, 2), 5), Some(RectU32::of(1, u32::MAX, 5, 3)));
    assert_eq!(try_wrapping_resize(&RectU32::of(u32::MAX - 2, u32::MAX - 4, u32::MAX, u32::MAX - 2), 5), Some(RectU32::of(u32::MAX - 3, u32::MAX - 5, 0, u32::MAX - 1)));
    assert_eq!(try_wrapping_resize(&RectU32::of(u32::MAX - 4, u32::MAX - 2, u32::MAX - 2, u32::MAX), 5), Some(RectU32::of(u32::MAX - 5, u32::MAX - 3, u32::MAX - 1, 0)));
}

#[test]
fn small_rect_limits_out_of_bounds() {
    assert_eq!(try_wrapping_resize(&RectU32::of(0, 2, 2, 4), u32::MAX), Some(RectU32::of(u32::MAX / 2 + 3, u32::MAX / 2 + 5, u32::MAX / 2 + 1, u32::MAX / 2 + 3)));
    assert_eq!(try_wrapping_resize(&RectU32::of(2, 0, 4, 2), u32::MAX), Some(RectU32::of(u32::MAX / 2 + 5, u32::MAX / 2 + 3, u32::MAX / 2 + 3, u32::MAX / 2 + 1)));
    assert_eq!(try_wrapping_resize(&RectU32::of(u32::MAX - 2, u32::MAX - 4, u32::MAX, u32::MAX - 2), u32::MAX), Some(RectU32::of(u32::MAX / 2, u32::MAX / 2 - 2, u32::MAX / 2 - 2, u32::MAX / 2 - 4)));
    assert_eq!(try_wrapping_resize(&RectU32::of(u32::MAX - 4, u32::MAX - 2, u32::MAX - 2, u32::MAX), u32::MAX), Some(RectU32::of(u32::MAX / 2 - 2, u32::MAX / 2, u32::MAX / 2 - 4, u32::MAX / 2 - 2)));
}

#[test]
fn big_rect_limits_out_of_bounds() {
    assert_eq!(try_wrapping_resize(&RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1), u32::MAX), Some(RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1)));
    assert_eq!(try_wrapping_resize(&RectU32::of(1, 1, u32::MAX, u32::MAX), u32::MAX), Some(RectU32::of(1, 1, u32::MAX, u32::MAX)));
    assert_eq!(try_wrapping_resize(&RectU32::largest(), u32::MAX), Some(RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1)));
}

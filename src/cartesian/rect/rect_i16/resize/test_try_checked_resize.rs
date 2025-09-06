use super::try_checked_resize;
use crate::cartesian::rect::rect_i16::RectI16;

#[test]
fn odd() {
    assert_eq!(try_checked_resize(&RectI16::of(-5, -5, 5, 5), 9), Some(RectI16::of(-4, -4, 4, 4)));
    assert_eq!(try_checked_resize(&RectI16::of(-4, -4, 4, 4), 7), Some(RectI16::of(-3, -3, 3, 3)));
    assert_eq!(try_checked_resize(&RectI16::of(-3, -3, 3, 3), 5), Some(RectI16::of(-2, -2, 2, 2)));
    assert_eq!(try_checked_resize(&RectI16::of(-2, -2, 2, 2), 3), Some(RectI16::of(-1, -1, 1, 1)));
    assert_eq!(try_checked_resize(&RectI16::of(-1, -1, 1, 1), 9), Some(RectI16::of(-4, -4, 4, 4)));
}

#[test]
fn even() {
    assert_eq!(try_checked_resize(&RectI16::of(-5, -5, 4, 4), 10), Some(RectI16::of(-5, -5, 4, 4)));
    assert_eq!(try_checked_resize(&RectI16::of(-5, -5, 4, 4), 8), Some(RectI16::of(-4, -4, 3, 3)));
    assert_eq!(try_checked_resize(&RectI16::of(-4, -4, 3, 3), 6), Some(RectI16::of(-3, -3, 2, 2)));
    assert_eq!(try_checked_resize(&RectI16::of(-3, -3, 2, 2), 4), Some(RectI16::of(-2, -2, 1, 1)));
    assert_eq!(try_checked_resize(&RectI16::of(-2, -2, 1, 1), 8), Some(RectI16::of(-4, -4, 3, 3)));
}

#[test]
fn small_size() {
    let r = RectI16::of(10, 10, 20, 20);
    assert_eq!(try_checked_resize(&r, 0), None);
    assert_eq!(try_checked_resize(&r, 1), None);
    assert_eq!(try_checked_resize(&r, 2), None);
}

#[test]
fn same_size() {
    assert_eq!(try_checked_resize(&RectI16::of(i16::MIN, i16::MIN, i16::MIN + 2, i16::MIN + 2), 3), Some(RectI16::of(i16::MIN, i16::MIN, i16::MIN + 2, i16::MIN + 2)));
    assert_eq!(try_checked_resize(&RectI16::of(i16::MIN, i16::MIN, i16::MIN + 3, i16::MIN + 3), 4), Some(RectI16::of(i16::MIN, i16::MIN, i16::MIN + 3, i16::MIN + 3)));
    assert_eq!(try_checked_resize(&RectI16::of(i16::MAX - 2, i16::MAX - 2, i16::MAX, i16::MAX), 3), Some(RectI16::of(i16::MAX - 2, i16::MAX - 2, i16::MAX, i16::MAX)));
    assert_eq!(try_checked_resize(&RectI16::of(i16::MAX - 3, i16::MAX - 3, i16::MAX, i16::MAX), 4), Some(RectI16::of(i16::MAX - 3, i16::MAX - 3, i16::MAX, i16::MAX)));
}

#[test]
fn out_of_bounds() {
    assert_eq!(try_checked_resize(&RectI16::of(i16::MIN, i16::MIN + 2, i16::MIN + 2, i16::MIN + 4), 5), None);
    assert_eq!(try_checked_resize(&RectI16::of(i16::MIN + 2, i16::MIN, i16::MIN + 4, i16::MIN + 2), 5), None);
    assert_eq!(try_checked_resize(&RectI16::of(i16::MAX - 2, i16::MAX - 4, i16::MAX, i16::MAX - 2), 5), None);
    assert_eq!(try_checked_resize(&RectI16::of(i16::MAX - 4, i16::MAX - 2, i16::MAX - 2, i16::MAX), 5), None);
}

#[test]
fn small_rect_limits_out_of_bounds() {
    assert_eq!(try_checked_resize(&RectI16::of(i16::MIN, i16::MIN + 2, i16::MIN + 2, i16::MIN + 4), u16::MAX), None);
    assert_eq!(try_checked_resize(&RectI16::of(i16::MIN + 2, i16::MIN, i16::MIN + 4, i16::MIN + 2), u16::MAX), None);
    assert_eq!(try_checked_resize(&RectI16::of(i16::MAX - 2, i16::MAX - 4, i16::MAX, i16::MAX - 2), u16::MAX), None);
    assert_eq!(try_checked_resize(&RectI16::of(i16::MAX - 4, i16::MAX - 2, i16::MAX - 2, i16::MAX), u16::MAX), None);
}

#[test]
fn big_rect_limits_out_of_bounds() {
    assert_eq!(try_checked_resize(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1), u16::MAX), Some(RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1)));
    assert_eq!(try_checked_resize(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX), u16::MAX), Some(RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX)));
    assert_eq!(try_checked_resize(&RectI16::largest(), u16::MAX), Some(RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1)));
}

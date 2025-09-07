use super::try_wrapping_resize;
use crate::cartesian::rect::rect_i16::RectI16;

const MIN: i16 = i16::MIN;
const MAX: i16 = i16::MAX;

#[test]
fn odd() {
    assert_eq!(try_wrapping_resize(&RectI16::of(-5, -5, 5, 5), 9), Some(RectI16::of(-4, -4, 4, 4)));
    assert_eq!(try_wrapping_resize(&RectI16::of(-4, -4, 4, 4), 7), Some(RectI16::of(-3, -3, 3, 3)));
    assert_eq!(try_wrapping_resize(&RectI16::of(-3, -3, 3, 3), 5), Some(RectI16::of(-2, -2, 2, 2)));
    assert_eq!(try_wrapping_resize(&RectI16::of(-2, -2, 2, 2), 3), Some(RectI16::of(-1, -1, 1, 1)));
    assert_eq!(try_wrapping_resize(&RectI16::of(-1, -1, 1, 1), 9), Some(RectI16::of(-4, -4, 4, 4)));
}

#[test]
fn even() {
    assert_eq!(try_wrapping_resize(&RectI16::of(-5, -5, 4, 4), 10), Some(RectI16::of(-5, -5, 4, 4)));
    assert_eq!(try_wrapping_resize(&RectI16::of(-5, -5, 4, 4), 8), Some(RectI16::of(-4, -4, 3, 3)));
    assert_eq!(try_wrapping_resize(&RectI16::of(-4, -4, 3, 3), 6), Some(RectI16::of(-3, -3, 2, 2)));
    assert_eq!(try_wrapping_resize(&RectI16::of(-3, -3, 2, 2), 4), Some(RectI16::of(-2, -2, 1, 1)));
    assert_eq!(try_wrapping_resize(&RectI16::of(-2, -2, 1, 1), 8), Some(RectI16::of(-4, -4, 3, 3)));
}

#[test]
fn small_size() {
    let r = RectI16::of(10, 10, 20, 20);
    assert_eq!(try_wrapping_resize(&r, 0), None);
    assert_eq!(try_wrapping_resize(&r, 1), None);
    assert_eq!(try_wrapping_resize(&r, 2), None);
}

#[test]
fn same_size() {
    assert_eq!(try_wrapping_resize(&RectI16::of(MIN, MIN, MIN + 2, MIN + 2), 3), Some(RectI16::of(MIN, MIN, MIN + 2, MIN + 2)));
    assert_eq!(try_wrapping_resize(&RectI16::of(MIN, MIN, MIN + 3, MIN + 3), 4), Some(RectI16::of(MIN, MIN, MIN + 3, MIN + 3)));
    assert_eq!(try_wrapping_resize(&RectI16::of(MAX - 2, MAX - 2, MAX, MAX), 3), Some(RectI16::of(MAX - 2, MAX - 2, MAX, MAX)));
    assert_eq!(try_wrapping_resize(&RectI16::of(MAX - 3, MAX - 3, MAX, MAX), 4), Some(RectI16::of(MAX - 3, MAX - 3, MAX, MAX)));
}

#[test]
fn out_of_bounds() {
    assert_eq!(try_wrapping_resize(&RectI16::of(MIN, MIN + 2, MIN + 2, MIN + 4), 5), Some(RectI16::of(MAX, MIN + 1, MIN + 3, MIN + 5)));
    assert_eq!(try_wrapping_resize(&RectI16::of(MIN + 2, MIN, MIN + 4, MIN + 2), 5), Some(RectI16::of(MIN + 1, MAX, MIN + 5, MIN + 3)));
    assert_eq!(try_wrapping_resize(&RectI16::of(MAX - 2, MAX - 4, MAX, MAX - 2), 5), Some(RectI16::of(MAX - 3, MAX - 5, MIN, MAX - 1)));
    assert_eq!(try_wrapping_resize(&RectI16::of(MAX - 4, MAX - 2, MAX - 2, MAX), 5), Some(RectI16::of(MAX - 5, MAX - 3, MAX - 1, MIN)));
}

#[test]
fn small_rect_limits_out_of_bounds() {
    assert_eq!(try_wrapping_resize(&RectI16::of(MIN, MIN + 2, MIN + 2, MIN + 4), u16::MAX), Some(RectI16::of(2, 4, 0, 2)));
    assert_eq!(try_wrapping_resize(&RectI16::of(MIN + 2, MIN, MIN + 4, MIN + 2), u16::MAX), Some(RectI16::of(4, 2, 2, 0)));
    assert_eq!(try_wrapping_resize(&RectI16::of(MAX - 2, MAX - 4, MAX, MAX - 2), u16::MAX), Some(RectI16::of(-1, -3, -3, -5)));
    assert_eq!(try_wrapping_resize(&RectI16::of(MAX - 4, MAX - 2, MAX - 2, MAX), u16::MAX), Some(RectI16::of(-3, -1, -5, -3)));
}

#[test]
fn big_rect_limits_out_of_bounds() {
    assert_eq!(try_wrapping_resize(&RectI16::of(MIN, MIN, MAX - 1, MAX - 1), u16::MAX), Some(RectI16::of(MIN, MIN, MAX - 1, MAX - 1)));
    assert_eq!(try_wrapping_resize(&RectI16::of(MIN + 1, MIN + 1, MAX, MAX), u16::MAX), Some(RectI16::of(MIN + 1, MIN + 1, MAX, MAX)));
    assert_eq!(try_wrapping_resize(&RectI16::largest(), u16::MAX), Some(RectI16::of(MIN, MIN, MAX - 1, MAX - 1)));
}

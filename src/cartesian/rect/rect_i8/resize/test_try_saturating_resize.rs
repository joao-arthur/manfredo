use super::try_saturating_resize;
use crate::cartesian::rect::rect_i8::RectI8;

const MIN: i8 = i8::MIN;
const MAX: i8 = i8::MAX;

#[test]
fn odd() {
    assert_eq!(try_saturating_resize(&RectI8::of(-5, -5, 5, 5), 9), Some(RectI8::of(-4, -4, 4, 4)));
    assert_eq!(try_saturating_resize(&RectI8::of(-4, -4, 4, 4), 7), Some(RectI8::of(-3, -3, 3, 3)));
    assert_eq!(try_saturating_resize(&RectI8::of(-3, -3, 3, 3), 5), Some(RectI8::of(-2, -2, 2, 2)));
    assert_eq!(try_saturating_resize(&RectI8::of(-2, -2, 2, 2), 3), Some(RectI8::of(-1, -1, 1, 1)));
    assert_eq!(try_saturating_resize(&RectI8::of(-1, -1, 1, 1), 9), Some(RectI8::of(-4, -4, 4, 4)));
}

#[test]
fn even() {
    assert_eq!(try_saturating_resize(&RectI8::of(-5, -5, 4, 4), 10), Some(RectI8::of(-5, -5, 4, 4)));
    assert_eq!(try_saturating_resize(&RectI8::of(-5, -5, 4, 4), 8), Some(RectI8::of(-4, -4, 3, 3)));
    assert_eq!(try_saturating_resize(&RectI8::of(-4, -4, 3, 3), 6), Some(RectI8::of(-3, -3, 2, 2)));
    assert_eq!(try_saturating_resize(&RectI8::of(-3, -3, 2, 2), 4), Some(RectI8::of(-2, -2, 1, 1)));
    assert_eq!(try_saturating_resize(&RectI8::of(-2, -2, 1, 1), 8), Some(RectI8::of(-4, -4, 3, 3)));
}

#[test]
fn small_size() {
    let r = RectI8::of(10, 10, 20, 20);
    assert_eq!(try_saturating_resize(&r, 0), None);
    assert_eq!(try_saturating_resize(&r, 1), None);
    assert_eq!(try_saturating_resize(&r, 2), None);
}

#[test]
fn same_size() {
    assert_eq!(try_saturating_resize(&RectI8::of(MIN, MIN, MIN + 2, MIN + 2), 3), Some(RectI8::of(MIN, MIN, MIN + 2, MIN + 2)));
    assert_eq!(try_saturating_resize(&RectI8::of(MIN, MIN, MIN + 3, MIN + 3), 4), Some(RectI8::of(MIN, MIN, MIN + 3, MIN + 3)));
    assert_eq!(try_saturating_resize(&RectI8::of(MAX - 2, MAX - 2, MAX, MAX), 3), Some(RectI8::of(MAX - 2, MAX - 2, MAX, MAX)));
    assert_eq!(try_saturating_resize(&RectI8::of(MAX - 3, MAX - 3, MAX, MAX), 4), Some(RectI8::of(MAX - 3, MAX - 3, MAX, MAX)));
}

#[test]
fn bounds() {
    assert_eq!(try_saturating_resize(&RectI8::of(MIN, MIN, MIN + 2, MIN + 2), 11), Some(RectI8::of(MIN, MIN, MIN + 10, MIN + 10)));
    assert_eq!(try_saturating_resize(&RectI8::of(MAX - 2, MAX - 2, MAX, MAX), 11), Some(RectI8::of(MAX - 10, MAX - 10, MAX, MAX)));
}

#[test]
fn small_rect_limits() {
    assert_eq!(try_saturating_resize(&RectI8::of(MIN, MIN, MIN + 2, MIN + 2), u8::MAX), Some(RectI8::of(MIN, MIN, MAX - 1, MAX - 1)));
    assert_eq!(try_saturating_resize(&RectI8::of(MAX - 2, MAX - 2, MAX, MAX), u8::MAX), Some(RectI8::of(MIN + 1, MIN + 1, MAX, MAX)));
}

#[test]
fn big_rect_limits() {
    assert_eq!(try_saturating_resize(&RectI8::of(MIN, MIN, MAX - 1, MAX - 1), u8::MAX), Some(RectI8::of(MIN, MIN, MAX - 1, MAX - 1)));
    assert_eq!(try_saturating_resize(&RectI8::of(MIN + 1, MIN + 1, MAX, MAX), u8::MAX), Some(RectI8::of(MIN + 1, MIN + 1, MAX, MAX)));
    assert_eq!(try_saturating_resize(&RectI8::largest(), u8::MAX), Some(RectI8::of(MIN, MIN, MAX - 1, MAX - 1)));
}

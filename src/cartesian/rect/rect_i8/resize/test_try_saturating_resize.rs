use super::try_saturating_resize;
use crate::cartesian::rect::rect_i8::RectI8;

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
    assert_eq!(try_saturating_resize(&RectI8::of(i8::MIN, i8::MIN, i8::MIN + 2, i8::MIN + 2), 3), Some(RectI8::of(i8::MIN, i8::MIN, i8::MIN + 2, i8::MIN + 2)));
    assert_eq!(try_saturating_resize(&RectI8::of(i8::MIN, i8::MIN, i8::MIN + 3, i8::MIN + 3), 4), Some(RectI8::of(i8::MIN, i8::MIN, i8::MIN + 3, i8::MIN + 3)));
    assert_eq!(try_saturating_resize(&RectI8::of(i8::MAX - 2, i8::MAX - 2, i8::MAX, i8::MAX), 3), Some(RectI8::of(i8::MAX - 2, i8::MAX - 2, i8::MAX, i8::MAX)));
    assert_eq!(try_saturating_resize(&RectI8::of(i8::MAX - 3, i8::MAX - 3, i8::MAX, i8::MAX), 4), Some(RectI8::of(i8::MAX - 3, i8::MAX - 3, i8::MAX, i8::MAX)));
}

#[test]
fn bounds() {
    assert_eq!(try_saturating_resize(&RectI8::of(i8::MIN, i8::MIN, i8::MIN + 2, i8::MIN + 2), 11), Some(RectI8::of(i8::MIN, i8::MIN, i8::MIN + 10, i8::MIN + 10)));
    assert_eq!(try_saturating_resize(&RectI8::of(i8::MAX - 2, i8::MAX - 2, i8::MAX, i8::MAX), 11), Some(RectI8::of(i8::MAX - 10, i8::MAX - 10, i8::MAX, i8::MAX)));
}

#[test]
fn small_rect_limits() {
    assert_eq!(try_saturating_resize(&RectI8::of(i8::MIN, i8::MIN, i8::MIN + 2, i8::MIN + 2), u8::MAX), Some(RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1)));
    assert_eq!(try_saturating_resize(&RectI8::of(i8::MAX - 2, i8::MAX - 2, i8::MAX, i8::MAX), u8::MAX), Some(RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX)));
}

#[test]
fn big_rect_limits() {
    assert_eq!(try_saturating_resize(&RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1), u8::MAX), Some(RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1)));
    assert_eq!(try_saturating_resize(&RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX), u8::MAX), Some(RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX)));
    assert_eq!(try_saturating_resize(&RectI8::largest(), u8::MAX), Some(RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1)));
}

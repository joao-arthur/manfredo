use super::try_saturating_resize;
use crate::cartesian::rect::rect_i32::RectI32;

#[test]
fn odd() {
    assert_eq!(try_saturating_resize(&RectI32::of(-5, -5, 5, 5), 9), Some(RectI32::of(-4, -4, 4, 4)));
    assert_eq!(try_saturating_resize(&RectI32::of(-4, -4, 4, 4), 7), Some(RectI32::of(-3, -3, 3, 3)));
    assert_eq!(try_saturating_resize(&RectI32::of(-3, -3, 3, 3), 5), Some(RectI32::of(-2, -2, 2, 2)));
    assert_eq!(try_saturating_resize(&RectI32::of(-2, -2, 2, 2), 3), Some(RectI32::of(-1, -1, 1, 1)));
    assert_eq!(try_saturating_resize(&RectI32::of(-1, -1, 1, 1), 9), Some(RectI32::of(-4, -4, 4, 4)));
}

#[test]
fn even() {
    assert_eq!(try_saturating_resize(&RectI32::of(-5, -5, 4, 4), 10), Some(RectI32::of(-5, -5, 4, 4)));
    assert_eq!(try_saturating_resize(&RectI32::of(-5, -5, 4, 4), 8), Some(RectI32::of(-4, -4, 3, 3)));
    assert_eq!(try_saturating_resize(&RectI32::of(-4, -4, 3, 3), 6), Some(RectI32::of(-3, -3, 2, 2)));
    assert_eq!(try_saturating_resize(&RectI32::of(-3, -3, 2, 2), 4), Some(RectI32::of(-2, -2, 1, 1)));
    assert_eq!(try_saturating_resize(&RectI32::of(-2, -2, 1, 1), 8), Some(RectI32::of(-4, -4, 3, 3)));
}

#[test]
fn small_size() {
    let r = RectI32::of(10, 10, 20, 20);
    assert_eq!(try_saturating_resize(&r, 0), None);
    assert_eq!(try_saturating_resize(&r, 1), None);
    assert_eq!(try_saturating_resize(&r, 2), None);
}

#[test]
fn same_size() {
    assert_eq!(try_saturating_resize(&RectI32::of(i32::MIN, i32::MIN, i32::MIN + 2, i32::MIN + 2), 3), Some(RectI32::of(i32::MIN, i32::MIN, i32::MIN + 2, i32::MIN + 2)));
    assert_eq!(try_saturating_resize(&RectI32::of(i32::MIN, i32::MIN, i32::MIN + 3, i32::MIN + 3), 4), Some(RectI32::of(i32::MIN, i32::MIN, i32::MIN + 3, i32::MIN + 3)));
    assert_eq!(try_saturating_resize(&RectI32::of(i32::MAX - 2, i32::MAX - 2, i32::MAX, i32::MAX), 3), Some(RectI32::of(i32::MAX - 2, i32::MAX - 2, i32::MAX, i32::MAX)));
    assert_eq!(try_saturating_resize(&RectI32::of(i32::MAX - 3, i32::MAX - 3, i32::MAX, i32::MAX), 4), Some(RectI32::of(i32::MAX - 3, i32::MAX - 3, i32::MAX, i32::MAX)));
}

#[test]
fn bounds() {
    assert_eq!(try_saturating_resize(&RectI32::of(i32::MIN, i32::MIN, i32::MIN + 2, i32::MIN + 2), 11), Some(RectI32::of(i32::MIN, i32::MIN, i32::MIN + 10, i32::MIN + 10)));
    assert_eq!(try_saturating_resize(&RectI32::of(i32::MAX - 2, i32::MAX - 2, i32::MAX, i32::MAX), 11), Some(RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX, i32::MAX)));
}

#[test]
fn small_rect_limits() {
    assert_eq!(try_saturating_resize(&RectI32::of(i32::MIN, i32::MIN, i32::MIN + 2, i32::MIN + 2), u32::MAX), Some(RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1)));
    assert_eq!(try_saturating_resize(&RectI32::of(i32::MAX - 2, i32::MAX - 2, i32::MAX, i32::MAX), u32::MAX), Some(RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX)));
}

#[test]
fn big_rect_limits() {
    assert_eq!(try_saturating_resize(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1), u32::MAX), Some(RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1)));
    assert_eq!(try_saturating_resize(&RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX), u32::MAX), Some(RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX)));
    assert_eq!(try_saturating_resize(&RectI32::largest(), u32::MAX), Some(RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1)));
}

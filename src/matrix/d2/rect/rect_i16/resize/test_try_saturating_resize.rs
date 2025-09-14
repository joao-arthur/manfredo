use super::try_saturating_resize;
use crate::matrix::rect::rect_i16::Rect;

const MIN: i16 = i16::MIN;
const MAX: i16 = i16::MAX;

#[test]
fn odd() {
    assert_eq!(try_saturating_resize(&Rect::of(-5, -5, 5, 5), 9), Some(Rect::of(-4, -4, 4, 4)));
    assert_eq!(try_saturating_resize(&Rect::of(-4, -4, 4, 4), 7), Some(Rect::of(-3, -3, 3, 3)));
    assert_eq!(try_saturating_resize(&Rect::of(-3, -3, 3, 3), 5), Some(Rect::of(-2, -2, 2, 2)));
    assert_eq!(try_saturating_resize(&Rect::of(-2, -2, 2, 2), 3), Some(Rect::of(-1, -1, 1, 1)));
    assert_eq!(try_saturating_resize(&Rect::of(-1, -1, 1, 1), 9), Some(Rect::of(-4, -4, 4, 4)));
}

#[test]
fn even() {
    assert_eq!(try_saturating_resize(&Rect::of(-5, -5, 4, 4), 10), Some(Rect::of(-5, -5, 4, 4)));
    assert_eq!(try_saturating_resize(&Rect::of(-5, -5, 4, 4), 8), Some(Rect::of(-4, -4, 3, 3)));
    assert_eq!(try_saturating_resize(&Rect::of(-4, -4, 3, 3), 6), Some(Rect::of(-3, -3, 2, 2)));
    assert_eq!(try_saturating_resize(&Rect::of(-3, -3, 2, 2), 4), Some(Rect::of(-2, -2, 1, 1)));
    assert_eq!(try_saturating_resize(&Rect::of(-2, -2, 1, 1), 8), Some(Rect::of(-4, -4, 3, 3)));
}

#[test]
fn small_size() {
    let r = Rect::of(10, 10, 20, 20);
    assert_eq!(try_saturating_resize(&r, 0), None);
    assert_eq!(try_saturating_resize(&r, 1), None);
    assert_eq!(try_saturating_resize(&r, 2), None);
}

#[test]
fn same_size() {
    assert_eq!(try_saturating_resize(&Rect::of(MIN, MIN, MIN + 2, MIN + 2), 3), Some(Rect::of(MIN, MIN, MIN + 2, MIN + 2)));
    assert_eq!(try_saturating_resize(&Rect::of(MIN, MIN, MIN + 3, MIN + 3), 4), Some(Rect::of(MIN, MIN, MIN + 3, MIN + 3)));
    assert_eq!(try_saturating_resize(&Rect::of(MAX - 2, MAX - 2, MAX, MAX), 3), Some(Rect::of(MAX - 2, MAX - 2, MAX, MAX)));
    assert_eq!(try_saturating_resize(&Rect::of(MAX - 3, MAX - 3, MAX, MAX), 4), Some(Rect::of(MAX - 3, MAX - 3, MAX, MAX)));
}

#[test]
fn bounds() {
    assert_eq!(try_saturating_resize(&Rect::of(MIN, MIN, MIN + 2, MIN + 2), 11), Some(Rect::of(MIN, MIN, MIN + 10, MIN + 10)));
    assert_eq!(try_saturating_resize(&Rect::of(MAX - 2, MAX - 2, MAX, MAX), 11), Some(Rect::of(MAX - 10, MAX - 10, MAX, MAX)));
}

#[test]
fn small_rect_limits() {
    assert_eq!(try_saturating_resize(&Rect::of(MIN, MIN, MIN + 2, MIN + 2), u16::MAX), Some(Rect::of(MIN, MIN, MAX - 1, MAX - 1)));
    assert_eq!(try_saturating_resize(&Rect::of(MAX - 2, MAX - 2, MAX, MAX), u16::MAX), Some(Rect::of(MIN + 1, MIN + 1, MAX, MAX)));
}

#[test]
fn big_rect_limits() {
    assert_eq!(try_saturating_resize(&Rect::of(MIN, MIN, MAX - 1, MAX - 1), u16::MAX), Some(Rect::of(MIN, MIN, MAX - 1, MAX - 1)));
    assert_eq!(try_saturating_resize(&Rect::of(MIN + 1, MIN + 1, MAX, MAX), u16::MAX), Some(Rect::of(MIN + 1, MIN + 1, MAX, MAX)));
    assert_eq!(try_saturating_resize(&Rect::largest(), u16::MAX), Some(Rect::of(MIN, MIN, MAX - 1, MAX - 1)));
}

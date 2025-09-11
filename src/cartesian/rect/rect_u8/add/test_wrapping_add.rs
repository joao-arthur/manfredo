use super::wrapping_add;
use crate::cartesian::rect::{rect_i8::Rect, rect_u8::RectU8};

const MAX: u8 = u8::MAX;

#[test]
fn test() {
    assert_eq!(wrapping_add(&RectU8::of(0, 0, 12, 15), &Rect::of(5, 4, 3, 2)), RectU8::of(5, 4, 15, 17));
    assert_eq!(wrapping_add(&RectU8::of(5, 4, 15, 20), &Rect::of(-4, -3, -2, -1)), RectU8::of(1, 1, 13, 19));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_add(&RectU8::of(2, 5, MAX - 2, MAX - 5), &Rect::of(-2, -5, 2, 5)), RectU8::largest());
    assert_eq!(wrapping_add(&RectU8::of(2, 5, MAX, MAX), &Rect::of(-2, -5, 0, 0)), RectU8::largest());
    assert_eq!(wrapping_add(&RectU8::of(0, 0, MAX - 2, MAX - 5), &Rect::of(0, 0, 2, 5)), RectU8::largest());
}

#[test]
fn out_of_bounds() {
    let r = RectU8::of(10, 10, MAX - 10, MAX - 10);
    assert_eq!(wrapping_add(&r, &Rect::of(-20, 0, 0, 0)), RectU8::of(MAX - 9, 10, MAX - 10, MAX - 10));
    assert_eq!(wrapping_add(&r, &Rect::of(0, -20, 0, 0)), RectU8::of(10, MAX - 9, MAX - 10, MAX - 10));
    assert_eq!(wrapping_add(&r, &Rect::of(0, 0, 20, 0)), RectU8::of(10, 10, 9, MAX - 10));
    assert_eq!(wrapping_add(&r, &Rect::of(0, 0, 0, 20)), RectU8::of(10, 10, MAX - 10, 9));
}

#[test]
fn edge_out_of_bounds() {
    let r = RectU8::largest();
    assert_eq!(wrapping_add(&r, &Rect::of(-1, 0, 0, 0)), RectU8::of(MAX, 0, MAX, MAX));
    assert_eq!(wrapping_add(&r, &Rect::of(0, -1, 0, 0)), RectU8::of(0, MAX, MAX, MAX));
    assert_eq!(wrapping_add(&r, &Rect::of(0, 0, 1, 0)), RectU8::of(0, 0, 0, MAX));
    assert_eq!(wrapping_add(&r, &Rect::of(0, 0, 0, 1)), RectU8::of(0, 0, MAX, 0));
}

#[test]
fn limits_out_of_bounds() {
    let r = RectU8::largest();
    assert_eq!(wrapping_add(&r, &Rect::of(i8::MIN, 0, 0, 0)), RectU8::of(MAX / 2 + 1, 0, MAX, MAX));
    assert_eq!(wrapping_add(&r, &Rect::of(0, i8::MIN, 0, 0)), RectU8::of(0, MAX / 2 + 1, MAX, MAX));
    assert_eq!(wrapping_add(&r, &Rect::of(0, 0, i8::MAX, 0)), RectU8::of(0, 0, MAX / 2 - 1, MAX));
    assert_eq!(wrapping_add(&r, &Rect::of(0, 0, 0, i8::MAX)), RectU8::of(0, 0, MAX, MAX / 2 - 1));
}

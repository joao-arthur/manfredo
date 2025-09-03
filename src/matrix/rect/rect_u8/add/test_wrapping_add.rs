use super::wrapping_add;
use crate::matrix::rect::{rect_i8::RectI8, rect_u8::RectU8};

#[test]
fn test_wrapping_add() {
    assert_eq!(wrapping_add(&RectU8::of(0, 0, 12, 15), &RectI8::of(5, 4, 3, 2)), RectU8::of(5, 4, 15, 17));
    assert_eq!(wrapping_add(&RectU8::of(5, 4, 15, 20), &RectI8::of(-4, -3, -2, -1)), RectU8::of(1, 1, 13, 19));
}

#[test]
fn wrapping_add_to_bounds() {
    assert_eq!(wrapping_add(&RectU8::of(2, 5, u8::MAX - 2, u8::MAX - 5), &RectI8::of(-2, -5, 2, 5)), RectU8::largest());
    assert_eq!(wrapping_add(&RectU8::of(2, 5, u8::MAX, u8::MAX), &RectI8::of(-2, -5, 0, 0)), RectU8::largest());
    assert_eq!(wrapping_add(&RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 5), &RectI8::of(0, 0, 2, 5)), RectU8::largest());
}

#[test]
fn wrapping_add_out_of_bounds() {
    let r = RectU8::of(10, 10, u8::MAX - 10, u8::MAX - 10);
    assert_eq!(wrapping_add(&r, &RectI8::of(-20, 0, 0, 0)), RectU8::of(u8::MAX - 9, 10, u8::MAX - 10, u8::MAX - 10));
    assert_eq!(wrapping_add(&r, &RectI8::of(0, -20, 0, 0)), RectU8::of(10, u8::MAX - 9, u8::MAX - 10, u8::MAX - 10));
    assert_eq!(wrapping_add(&r, &RectI8::of(0, 0, 20, 0)), RectU8::of(10, 10, 9, u8::MAX - 10));
    assert_eq!(wrapping_add(&r, &RectI8::of(0, 0, 0, 20)), RectU8::of(10, 10, u8::MAX - 10, 9));
}

#[test]
fn wrapping_add_edge_out_of_bounds() {
    let r = RectU8::largest();
    assert_eq!(wrapping_add(&r, &RectI8::of(-1, 0, 0, 0)), RectU8::of(u8::MAX, 0, u8::MAX, u8::MAX));
    assert_eq!(wrapping_add(&r, &RectI8::of(0, -1, 0, 0)), RectU8::of(0, u8::MAX, u8::MAX, u8::MAX));
    assert_eq!(wrapping_add(&r, &RectI8::of(0, 0, 1, 0)), RectU8::of(0, 0, 0, u8::MAX));
    assert_eq!(wrapping_add(&r, &RectI8::of(0, 0, 0, 1)), RectU8::of(0, 0, u8::MAX, 0));
}

#[test]
fn wrapping_add_limits_out_of_bounds() {
    let r = RectU8::largest();
    assert_eq!(wrapping_add(&r, &RectI8::of(i8::MIN, 0, 0, 0)), RectU8::of(u8::MAX / 2 + 1, 0, u8::MAX, u8::MAX));
    assert_eq!(wrapping_add(&r, &RectI8::of(0, i8::MIN, 0, 0)), RectU8::of(0, u8::MAX / 2 + 1, u8::MAX, u8::MAX));
    assert_eq!(wrapping_add(&r, &RectI8::of(0, 0, i8::MAX, 0)), RectU8::of(0, 0, u8::MAX / 2 - 1, u8::MAX));
    assert_eq!(wrapping_add(&r, &RectI8::of(0, 0, 0, i8::MAX)), RectU8::of(0, 0, u8::MAX, u8::MAX / 2 - 1));
}

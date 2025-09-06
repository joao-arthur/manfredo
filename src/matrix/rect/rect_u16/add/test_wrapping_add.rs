use super::wrapping_add;
use crate::matrix::rect::{rect_i16::RectI16, rect_u16::RectU16};

#[test]
fn test() {
    assert_eq!(wrapping_add(&RectU16::of(0, 0, 12, 15), &RectI16::of(5, 4, 3, 2)), RectU16::of(5, 4, 15, 17));
    assert_eq!(wrapping_add(&RectU16::of(5, 4, 15, 20), &RectI16::of(-4, -3, -2, -1)), RectU16::of(1, 1, 13, 19));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_add(&RectU16::of(2, 5, u16::MAX - 2, u16::MAX - 5), &RectI16::of(-2, -5, 2, 5)), RectU16::largest());
    assert_eq!(wrapping_add(&RectU16::of(2, 5, u16::MAX, u16::MAX), &RectI16::of(-2, -5, 0, 0)), RectU16::largest());
    assert_eq!(wrapping_add(&RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5), &RectI16::of(0, 0, 2, 5)), RectU16::largest());
}

#[test]
fn out_of_bounds() {
    let r = RectU16::of(10, 10, u16::MAX - 10, u16::MAX - 10);
    assert_eq!(wrapping_add(&r, &RectI16::of(-20, 0, 0, 0)), RectU16::of(u16::MAX - 9, 10, u16::MAX - 10, u16::MAX - 10));
    assert_eq!(wrapping_add(&r, &RectI16::of(0, -20, 0, 0)), RectU16::of(10, u16::MAX - 9, u16::MAX - 10, u16::MAX - 10));
    assert_eq!(wrapping_add(&r, &RectI16::of(0, 0, 20, 0)), RectU16::of(10, 10, 9, u16::MAX - 10));
    assert_eq!(wrapping_add(&r, &RectI16::of(0, 0, 0, 20)), RectU16::of(10, 10, u16::MAX - 10, 9));
}

#[test]
fn edge_out_of_bounds() {
    let r = RectU16::largest();
    assert_eq!(wrapping_add(&r, &RectI16::of(-1, 0, 0, 0)), RectU16::of(u16::MAX, 0, u16::MAX, u16::MAX));
    assert_eq!(wrapping_add(&r, &RectI16::of(0, -1, 0, 0)), RectU16::of(0, u16::MAX, u16::MAX, u16::MAX));
    assert_eq!(wrapping_add(&r, &RectI16::of(0, 0, 1, 0)), RectU16::of(0, 0, 0, u16::MAX));
    assert_eq!(wrapping_add(&r, &RectI16::of(0, 0, 0, 1)), RectU16::of(0, 0, u16::MAX, 0));
}

#[test]
fn limits_out_of_bounds() {
    let r = RectU16::largest();
    assert_eq!(wrapping_add(&r, &RectI16::of(i16::MIN, 0, 0, 0)), RectU16::of(u16::MAX / 2 + 1, 0, u16::MAX, u16::MAX));
    assert_eq!(wrapping_add(&r, &RectI16::of(0, i16::MIN, 0, 0)), RectU16::of(0, u16::MAX / 2 + 1, u16::MAX, u16::MAX));
    assert_eq!(wrapping_add(&r, &RectI16::of(0, 0, i16::MAX, 0)), RectU16::of(0, 0, u16::MAX / 2 - 1, u16::MAX));
    assert_eq!(wrapping_add(&r, &RectI16::of(0, 0, 0, i16::MAX)), RectU16::of(0, 0, u16::MAX, u16::MAX / 2 - 1));
}

use super::wrapping_add;
use crate::matrix::rect::{rect_i32::RectI32, rect_u32::RectU32};

#[test]
fn test() {
    assert_eq!(wrapping_add(&RectU32::of(0, 0, 12, 15), &RectI32::of(5, 4, 3, 2)), RectU32::of(5, 4, 15, 17));
    assert_eq!(wrapping_add(&RectU32::of(5, 4, 15, 20), &RectI32::of(-4, -3, -2, -1)), RectU32::of(1, 1, 13, 19));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_add(&RectU32::of(2, 5, u32::MAX - 2, u32::MAX - 5), &RectI32::of(-2, -5, 2, 5)), RectU32::largest());
    assert_eq!(wrapping_add(&RectU32::of(2, 5, u32::MAX, u32::MAX), &RectI32::of(-2, -5, 0, 0)), RectU32::largest());
    assert_eq!(wrapping_add(&RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5), &RectI32::of(0, 0, 2, 5)), RectU32::largest());
}

#[test]
fn out_of_bounds() {
    let r = RectU32::of(10, 10, u32::MAX - 10, u32::MAX - 10);
    assert_eq!(wrapping_add(&r, &RectI32::of(-20, 0, 0, 0)), RectU32::of(u32::MAX - 9, 10, u32::MAX - 10, u32::MAX - 10));
    assert_eq!(wrapping_add(&r, &RectI32::of(0, -20, 0, 0)), RectU32::of(10, u32::MAX - 9, u32::MAX - 10, u32::MAX - 10));
    assert_eq!(wrapping_add(&r, &RectI32::of(0, 0, 20, 0)), RectU32::of(10, 10, 9, u32::MAX - 10));
    assert_eq!(wrapping_add(&r, &RectI32::of(0, 0, 0, 20)), RectU32::of(10, 10, u32::MAX - 10, 9));
}

#[test]
fn edge_out_of_bounds() {
    let r = RectU32::largest();
    assert_eq!(wrapping_add(&r, &RectI32::of(-1, 0, 0, 0)), RectU32::of(u32::MAX, 0, u32::MAX, u32::MAX));
    assert_eq!(wrapping_add(&r, &RectI32::of(0, -1, 0, 0)), RectU32::of(0, u32::MAX, u32::MAX, u32::MAX));
    assert_eq!(wrapping_add(&r, &RectI32::of(0, 0, 1, 0)), RectU32::of(0, 0, 0, u32::MAX));
    assert_eq!(wrapping_add(&r, &RectI32::of(0, 0, 0, 1)), RectU32::of(0, 0, u32::MAX, 0));
}

#[test]
fn limits_out_of_bounds() {
    let r = RectU32::largest();
    assert_eq!(wrapping_add(&r, &RectI32::of(i32::MIN, 0, 0, 0)), RectU32::of(u32::MAX / 2 + 1, 0, u32::MAX, u32::MAX));
    assert_eq!(wrapping_add(&r, &RectI32::of(0, i32::MIN, 0, 0)), RectU32::of(0, u32::MAX / 2 + 1, u32::MAX, u32::MAX));
    assert_eq!(wrapping_add(&r, &RectI32::of(0, 0, i32::MAX, 0)), RectU32::of(0, 0, u32::MAX / 2 - 1, u32::MAX));
    assert_eq!(wrapping_add(&r, &RectI32::of(0, 0, 0, i32::MAX)), RectU32::of(0, 0, u32::MAX, u32::MAX / 2 - 1));
}

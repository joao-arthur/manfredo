use super::wrapping_add;
use crate::matrix::rect::rect_i32::RectI32;

const MIN: i32 = i32::MIN;
const MAX: i32 = i32::MAX;

#[test]
fn test() {
    assert_eq!(wrapping_add(&RectI32::of(-7, 9, -12, 15), &RectI32::of(5, 4, 3, 2)), RectI32::of(-2, 13, -9, 17));
    assert_eq!(wrapping_add(&RectI32::of(-2, 13, -9, 17), &RectI32::of(9, -10, 11, -12)), RectI32::of(7, 3, 2, 5));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_add(&RectI32::of(MIN + 2, MIN + 5, MAX - 2, MAX - 5), &RectI32::of(-2, -5, 2, 5)), RectI32::largest());
    assert_eq!(wrapping_add(&RectI32::of(MIN + 2, MIN + 5, MAX, MAX), &RectI32::of(-2, -5, 0, 0)), RectI32::largest());
    assert_eq!(wrapping_add(&RectI32::of(MIN, MIN, MAX - 2, MAX - 5), &RectI32::of(0, 0, 2, 5)), RectI32::largest());
}

#[test]
fn out_of_bounds() {
    let r = RectI32::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10);
    assert_eq!(wrapping_add(&r, &RectI32::of(-20, 0, 0, 0)), RectI32::of(MAX - 9, MIN + 10, MAX - 10, MAX - 10));
    assert_eq!(wrapping_add(&r, &RectI32::of(0, -20, 0, 0)), RectI32::of(MIN + 10, MAX - 9, MAX - 10, MAX - 10));
    assert_eq!(wrapping_add(&r, &RectI32::of(0, 0, 20, 0)), RectI32::of(MIN + 10, MIN + 10, MIN + 9, MAX - 10));
    assert_eq!(wrapping_add(&r, &RectI32::of(0, 0, 0, 20)), RectI32::of(MIN + 10, MIN + 10, MAX - 10, MIN + 9));
}

#[test]
fn edge_out_of_bounds() {
    let r = RectI32::largest();
    assert_eq!(wrapping_add(&r, &RectI32::of(-1, 0, 0, 0)), RectI32::of(MAX, MIN, MAX, MAX));
    assert_eq!(wrapping_add(&r, &RectI32::of(0, -1, 0, 0)), RectI32::of(MIN, MAX, MAX, MAX));
    assert_eq!(wrapping_add(&r, &RectI32::of(0, 0, 1, 0)), RectI32::of(MIN, MIN, MIN, MAX));
    assert_eq!(wrapping_add(&r, &RectI32::of(0, 0, 0, 1)), RectI32::of(MIN, MIN, MAX, MIN));
}

#[test]
fn limits_out_of_bounds() {
    let r = RectI32::largest();
    assert_eq!(wrapping_add(&r, &RectI32::of(MIN, 0, 0, 0)), RectI32::of(0, MIN, MAX, MAX));
    assert_eq!(wrapping_add(&r, &RectI32::of(0, MIN, 0, 0)), RectI32::of(MIN, 0, MAX, MAX));
    assert_eq!(wrapping_add(&r, &RectI32::of(0, 0, MAX, 0)), RectI32::of(MIN, MIN, -2, MAX));
    assert_eq!(wrapping_add(&r, &RectI32::of(0, 0, 0, MAX)), RectI32::of(MIN, MIN, MAX, -2));
}

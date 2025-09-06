use super::wrapping_add;
use crate::cartesian::rect::rect_i8::RectI8;

#[test]
fn test() {
    assert_eq!(wrapping_add(&RectI8::of(-7, 9, -12, 15), &RectI8::of(5, 4, 3, 2)), RectI8::of(-2, 13, -9, 17));
    assert_eq!(wrapping_add(&RectI8::of(-2, 13, -9, 17), &RectI8::of(9, -10, 11, -12)), RectI8::of(7, 3, 2, 5));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_add(&RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX - 2, i8::MAX - 5), &RectI8::of(-2, -5, 2, 5)), RectI8::largest());
    assert_eq!(wrapping_add(&RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX, i8::MAX), &RectI8::of(-2, -5, 0, 0)), RectI8::largest());
    assert_eq!(wrapping_add(&RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 5), &RectI8::of(0, 0, 2, 5)), RectI8::largest());
}

#[test]
fn out_of_bounds() {
    let r = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 10, i8::MAX - 10);
    assert_eq!(wrapping_add(&r, &RectI8::of(-20, 0, 0, 0)), RectI8::of(i8::MAX - 9, i8::MIN + 10, i8::MAX - 10, i8::MAX - 10));
    assert_eq!(wrapping_add(&r, &RectI8::of(0, -20, 0, 0)), RectI8::of(i8::MIN + 10, i8::MAX - 9, i8::MAX - 10, i8::MAX - 10));
    assert_eq!(wrapping_add(&r, &RectI8::of(0, 0, 20, 0)), RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MIN + 9, i8::MAX - 10));
    assert_eq!(wrapping_add(&r, &RectI8::of(0, 0, 0, 20)), RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 10, i8::MIN + 9));
}

#[test]
fn edge_out_of_bounds() {
    let r = RectI8::largest();
    assert_eq!(wrapping_add(&r, &RectI8::of(-1, 0, 0, 0)), RectI8::of(i8::MAX, i8::MIN, i8::MAX, i8::MAX));
    assert_eq!(wrapping_add(&r, &RectI8::of(0, -1, 0, 0)), RectI8::of(i8::MIN, i8::MAX, i8::MAX, i8::MAX));
    assert_eq!(wrapping_add(&r, &RectI8::of(0, 0, 1, 0)), RectI8::of(i8::MIN, i8::MIN, i8::MIN, i8::MAX));
    assert_eq!(wrapping_add(&r, &RectI8::of(0, 0, 0, 1)), RectI8::of(i8::MIN, i8::MIN, i8::MAX, i8::MIN));
}

#[test]
fn limits_out_of_bounds() {
    let r = RectI8::largest();
    assert_eq!(wrapping_add(&r, &RectI8::of(i8::MIN, 0, 0, 0)), RectI8::of(0, i8::MIN, i8::MAX, i8::MAX));
    assert_eq!(wrapping_add(&r, &RectI8::of(0, i8::MIN, 0, 0)), RectI8::of(i8::MIN, 0, i8::MAX, i8::MAX));
    assert_eq!(wrapping_add(&r, &RectI8::of(0, 0, i8::MAX, 0)), RectI8::of(i8::MIN, i8::MIN, -2, i8::MAX));
    assert_eq!(wrapping_add(&r, &RectI8::of(0, 0, 0, i8::MAX)), RectI8::of(i8::MIN, i8::MIN, i8::MAX, -2));
}

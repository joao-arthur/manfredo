use super::wrapping_add;
use crate::matrix::rect::rect_i16::RectI16;

#[test]
fn test() {
    assert_eq!(wrapping_add(&RectI16::of(-7, 9, -12, 15), &RectI16::of(5, 4, 3, 2)), RectI16::of(-2, 13, -9, 17));
    assert_eq!(wrapping_add(&RectI16::of(-2, 13, -9, 17), &RectI16::of(9, -10, 11, -12)), RectI16::of(7, 3, 2, 5));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_add(&RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX - 2, i16::MAX - 5), &RectI16::of(-2, -5, 2, 5)), RectI16::largest());
    assert_eq!(wrapping_add(&RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX), &RectI16::of(-2, -5, 0, 0)), RectI16::largest());
    assert_eq!(wrapping_add(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5), &RectI16::of(0, 0, 2, 5)), RectI16::largest());
}

#[test]
fn out_of_bounds() {
    let r = RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 10, i16::MAX - 10);
    assert_eq!(wrapping_add(&r, &RectI16::of(-20, 0, 0, 0)), RectI16::of(i16::MAX - 9, i16::MIN + 10, i16::MAX - 10, i16::MAX - 10));
    assert_eq!(wrapping_add(&r, &RectI16::of(0, -20, 0, 0)), RectI16::of(i16::MIN + 10, i16::MAX - 9, i16::MAX - 10, i16::MAX - 10));
    assert_eq!(wrapping_add(&r, &RectI16::of(0, 0, 20, 0)), RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MIN + 9, i16::MAX - 10));
    assert_eq!(wrapping_add(&r, &RectI16::of(0, 0, 0, 20)), RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 10, i16::MIN + 9));
}

#[test]
fn edge_out_of_bounds() {
    let r = RectI16::largest();
    assert_eq!(wrapping_add(&r, &RectI16::of(-1, 0, 0, 0)), RectI16::of(i16::MAX, i16::MIN, i16::MAX, i16::MAX));
    assert_eq!(wrapping_add(&r, &RectI16::of(0, -1, 0, 0)), RectI16::of(i16::MIN, i16::MAX, i16::MAX, i16::MAX));
    assert_eq!(wrapping_add(&r, &RectI16::of(0, 0, 1, 0)), RectI16::of(i16::MIN, i16::MIN, i16::MIN, i16::MAX));
    assert_eq!(wrapping_add(&r, &RectI16::of(0, 0, 0, 1)), RectI16::of(i16::MIN, i16::MIN, i16::MAX, i16::MIN));
}

#[test]
fn limits_out_of_bounds() {
    let r = RectI16::largest();
    assert_eq!(wrapping_add(&r, &RectI16::of(i16::MIN, 0, 0, 0)), RectI16::of(0, i16::MIN, i16::MAX, i16::MAX));
    assert_eq!(wrapping_add(&r, &RectI16::of(0, i16::MIN, 0, 0)), RectI16::of(i16::MIN, 0, i16::MAX, i16::MAX));
    assert_eq!(wrapping_add(&r, &RectI16::of(0, 0, i16::MAX, 0)), RectI16::of(i16::MIN, i16::MIN, -2, i16::MAX));
    assert_eq!(wrapping_add(&r, &RectI16::of(0, 0, 0, i16::MAX)), RectI16::of(i16::MIN, i16::MIN, i16::MAX, -2));
}

use super::saturating_add;
use crate::matrix::rect::rect_i8::RectI8;

const MIN: i8 = i8::MIN;
const MAX: i8 = i8::MAX;

#[test]
fn test() {
    assert_eq!(saturating_add(&RectI8::of(-7, 9, -12, 15), &RectI8::of(5, 4, 3, 2)), RectI8::of(-2, 13, -9, 17));
    assert_eq!(saturating_add(&RectI8::of(-2, 13, -9, 17), &RectI8::of(9, -10, 11, -12)), RectI8::of(7, 3, 2, 5));
}

#[test]
fn to_bounds() {
    assert_eq!(saturating_add(&RectI8::of(MIN + 2, MIN + 5, MAX - 2, MAX - 5), &RectI8::of(-2, -5, 2, 5)), RectI8::largest());
    assert_eq!(saturating_add(&RectI8::of(MIN + 2, MIN + 5, MAX, MAX), &RectI8::of(-2, -5, 0, 0)), RectI8::largest());
    assert_eq!(saturating_add(&RectI8::of(MIN, MIN, MAX - 2, MAX - 5), &RectI8::of(0, 0, 2, 5)), RectI8::largest());
}

#[test]
fn out_of_bounds() {
    let r = RectI8::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10);
    assert_eq!(saturating_add(&r, &RectI8::of(-20, 0, 0, 0)), RectI8::of(MIN, MIN + 10, MAX - 10, MAX - 10));
    assert_eq!(saturating_add(&r, &RectI8::of(0, -20, 0, 0)), RectI8::of(MIN + 10, MIN, MAX - 10, MAX - 10));
    assert_eq!(saturating_add(&r, &RectI8::of(0, 0, 20, 0)), RectI8::of(MIN + 10, MIN + 10, MAX, MAX - 10));
    assert_eq!(saturating_add(&r, &RectI8::of(0, 0, 0, 20)), RectI8::of(MIN + 10, MIN + 10, MAX - 10, MAX));
}

#[test]
fn edge_out_of_bounds() {
    let r = RectI8::largest();
    assert_eq!(saturating_add(&r, &RectI8::of(-1, 0, 0, 0)), RectI8::largest());
    assert_eq!(saturating_add(&r, &RectI8::of(0, -1, 0, 0)), RectI8::largest());
    assert_eq!(saturating_add(&r, &RectI8::of(0, 0, 1, 0)), RectI8::largest());
    assert_eq!(saturating_add(&r, &RectI8::of(0, 0, 0, 1)), RectI8::largest());
}

#[test]
fn limits_out_of_bounds() {
    let r = RectI8::largest();
    assert_eq!(saturating_add(&r, &RectI8::of(MIN, 0, 0, 0)), RectI8::largest());
    assert_eq!(saturating_add(&r, &RectI8::of(0, MIN, 0, 0)), RectI8::largest());
    assert_eq!(saturating_add(&r, &RectI8::of(0, 0, MAX, 0)), RectI8::largest());
    assert_eq!(saturating_add(&r, &RectI8::of(0, 0, 0, MAX)), RectI8::largest());
}

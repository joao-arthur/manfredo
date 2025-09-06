use super::saturating_add;
use crate::matrix::rect::rect_i32::RectI32;

#[test]
fn test() {
    assert_eq!(saturating_add(&RectI32::of(-7, 9, -12, 15), &RectI32::of(5, 4, 3, 2)), RectI32::of(-2, 13, -9, 17));
    assert_eq!(saturating_add(&RectI32::of(-2, 13, -9, 17), &RectI32::of(9, -10, 11, -12)), RectI32::of(7, 3, 2, 5));
}

#[test]
fn to_bounds() {
    assert_eq!(saturating_add(&RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX - 2, i32::MAX - 5), &RectI32::of(-2, -5, 2, 5)), RectI32::largest());
    assert_eq!(saturating_add(&RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX, i32::MAX), &RectI32::of(-2, -5, 0, 0)), RectI32::largest());
    assert_eq!(saturating_add(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 5), &RectI32::of(0, 0, 2, 5)), RectI32::largest());
}

#[test]
fn out_of_bounds() {
    let r = RectI32::of(i32::MIN + 10, i32::MIN + 10, i32::MAX - 10, i32::MAX - 10);
    assert_eq!(saturating_add(&r, &RectI32::of(-20, 0, 0, 0)), RectI32::of(i32::MIN, i32::MIN + 10, i32::MAX - 10, i32::MAX - 10));
    assert_eq!(saturating_add(&r, &RectI32::of(0, -20, 0, 0)), RectI32::of(i32::MIN + 10, i32::MIN, i32::MAX - 10, i32::MAX - 10));
    assert_eq!(saturating_add(&r, &RectI32::of(0, 0, 20, 0)), RectI32::of(i32::MIN + 10, i32::MIN + 10, i32::MAX, i32::MAX - 10));
    assert_eq!(saturating_add(&r, &RectI32::of(0, 0, 0, 20)), RectI32::of(i32::MIN + 10, i32::MIN + 10, i32::MAX - 10, i32::MAX));
}

#[test]
fn edge_out_of_bounds() {
    let r = RectI32::largest();
    assert_eq!(saturating_add(&r, &RectI32::of(-1, 0, 0, 0)), RectI32::largest());
    assert_eq!(saturating_add(&r, &RectI32::of(0, -1, 0, 0)), RectI32::largest());
    assert_eq!(saturating_add(&r, &RectI32::of(0, 0, 1, 0)), RectI32::largest());
    assert_eq!(saturating_add(&r, &RectI32::of(0, 0, 0, 1)), RectI32::largest());
}

#[test]
fn limits_out_of_bounds() {
    let r = RectI32::largest();
    assert_eq!(saturating_add(&r, &RectI32::of(i32::MIN, 0, 0, 0)), RectI32::largest());
    assert_eq!(saturating_add(&r, &RectI32::of(0, i32::MIN, 0, 0)), RectI32::largest());
    assert_eq!(saturating_add(&r, &RectI32::of(0, 0, i32::MAX, 0)), RectI32::largest());
    assert_eq!(saturating_add(&r, &RectI32::of(0, 0, 0, i32::MAX)), RectI32::largest());
}

use super::saturating_add;
use crate::cartesian::{
    point::point_f64::{MAX, MIN},
    rect::rect_f64::RectF64,
};

#[test]
fn test() {
    assert_eq!(saturating_add(&RectF64::of(-7.0, 9.0, -12.0, 15.0), &RectF64::of(5.0, 4.0, 3.0, 2.0)), RectF64::of(-2.0, 13.0, -9.0, 17.0));
    assert_eq!(saturating_add(&RectF64::of(-2.0, 13.0, -9.0, 17.0), &RectF64::of(9.0, -10.0, 11.0, -12.0)), RectF64::of(7.0, 3.0, 2.0, 5.0));
}

#[test]
fn to_bounds() {
    assert_eq!(saturating_add(&RectF64::of(MIN + 2.0, MIN + 5.0, MAX - 2.0, MAX - 5.0), &RectF64::of(-2.0, -5.0, 2.0, 5.0)), RectF64::largest());
    assert_eq!(saturating_add(&RectF64::of(MIN + 2.0, MIN + 5.0, MAX, MAX), &RectF64::of(-2.0, -5.0, 0.0, 0.0)), RectF64::largest());
    assert_eq!(saturating_add(&RectF64::of(MIN, MIN, MAX - 2.0, MAX - 5.0), &RectF64::of(0.0, 0.0, 2.0, 5.0)), RectF64::largest());
}

#[test]
fn out_of_bounds() {
    let r = RectF64::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX - 10.0);
    assert_eq!(saturating_add(&r, &RectF64::of(-20.0, 0.0, 0.0, 0.0)), RectF64::of(MIN, MIN + 10.0, MAX - 10.0, MAX - 10.0));
    assert_eq!(saturating_add(&r, &RectF64::of(0.0, -20.0, 0.0, 0.0)), RectF64::of(MIN + 10.0, MIN, MAX - 10.0, MAX - 10.0));
    assert_eq!(saturating_add(&r, &RectF64::of(0.0, 0.0, 20.0, 0.0)), RectF64::of(MIN + 10.0, MIN + 10.0, MAX, MAX - 10.0));
    assert_eq!(saturating_add(&r, &RectF64::of(0.0, 0.0, 0.0, 20.0)), RectF64::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX));
}

#[test]
fn edge_out_of_bounds() {
    let r = RectF64::largest();
    assert_eq!(saturating_add(&r, &RectF64::of(-1.0, 0.0, 0.0, 0.0)), RectF64::largest());
    assert_eq!(saturating_add(&r, &RectF64::of(0.0, -1.0, 0.0, 0.0)), RectF64::largest());
    assert_eq!(saturating_add(&r, &RectF64::of(0.0, 0.0, 1.0, 0.0)), RectF64::largest());
    assert_eq!(saturating_add(&r, &RectF64::of(0.0, 0.0, 0.0, 1.0)), RectF64::largest());
}

#[test]
fn limits_out_of_bounds() {
    let r = RectF64::largest();
    assert_eq!(saturating_add(&r, &RectF64::of(MIN, 0.0, 0.0, 0.0)), RectF64::largest());
    assert_eq!(saturating_add(&r, &RectF64::of(0.0, MIN, 0.0, 0.0)), RectF64::largest());
    assert_eq!(saturating_add(&r, &RectF64::of(0.0, 0.0, MAX, 0.0)), RectF64::largest());
    assert_eq!(saturating_add(&r, &RectF64::of(0.0, 0.0, 0.0, MAX)), RectF64::largest());
}
